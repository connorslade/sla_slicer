use std::{
    collections::HashMap,
    ops::Deref,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};

use anyhow::Result;
use parking_lot::{Mutex, RwLock};
use soon::Soon;

use crate::{
    commands::{Command, CommandTrait, DisconnectCommand},
    mqtt::{
        packets::{
            connect::ConnectPacket,
            connect_ack::{ConnectAckFlags, ConnectAckPacket, ConnectReturnCode},
            publish::{PublishFlags, PublishPacket},
            publish_ack::PublishAckPacket,
            subscribe::SubscribePacket,
            subscribe_ack::{SubscribeAckPacket, SubscribeReturnCode},
        },
        MqttHandler, MqttServer,
    },
    status::{Attributes, FullStatusData, Status, StatusData},
    Response,
};

pub struct MqttInner {
    server: Soon<Arc<MqttServer<Mqtt>>>,
    /// mainboard_id -> MqttClient
    clients: RwLock<HashMap<String, MqttClient>>,
    /// client_id -> mainboard_id
    client_ids: RwLock<HashMap<u64, String>>,
}

#[derive(Clone)]
pub struct Mqtt {
    inner: Arc<MqttInner>,
}

struct MqttClient {
    attributes: Attributes,
    status: Mutex<Status>,
    machine_id: String,
    client_id: Option<u64>,
    next_packet_id: AtomicU16,
}

impl MqttHandler for Mqtt {
    fn init(&self, server: Arc<MqttServer<Self>>) {
        self.server.replace(server);
    }

    fn on_connect(&self, client_id: u64, _packet: ConnectPacket) -> Result<ConnectAckPacket> {
        println!("Client `{client_id}` connected");

        Ok(ConnectAckPacket {
            flags: ConnectAckFlags::empty(),
            return_code: ConnectReturnCode::Accepted,
        })
    }

    fn on_subscribe(&self, client_id: u64, packet: SubscribePacket) -> Result<SubscribeAckPacket> {
        println!(
            "Client `{client_id}` subscribed to topics: {:?}",
            packet.filters
        );

        let mut return_codes = vec![SubscribeReturnCode::Failure; packet.filters.len()];
        if let Some((idx, mainboard_id, qos)) =
            packet
                .filters
                .iter()
                .enumerate()
                .find_map(|(idx, (topic, qos))| {
                    topic.strip_prefix("/sdcp/request/").map(|x| (idx, x, qos))
                })
        {
            if self.clients.read().get(mainboard_id).is_none() {
                eprintln!("Client `{mainboard_id}` does not exist.");
                return Ok(SubscribeAckPacket {
                    packet_id: packet.packet_id,
                    return_codes,
                });
            }

            return_codes[idx] = SubscribeReturnCode::Success(*qos);
            self.client_ids
                .write()
                .insert(client_id, mainboard_id.to_owned());
            self.clients
                .write()
                .get_mut(mainboard_id)
                .unwrap()
                .client_id = Some(client_id);
        }

        Ok(SubscribeAckPacket {
            packet_id: packet.packet_id,
            return_codes,
        })
    }

    fn on_publish(&self, client_id: u64, packet: PublishPacket) -> Result<()> {
        println!("Client `{client_id}` published to topic `{}`", packet.topic);

        if let Some(board_id) = packet.topic.strip_prefix("/sdcp/status/") {
            let status = serde_json::from_slice::<Response<StatusData>>(&packet.data)?;
            println!("{:?}", status.data.status);
            *self.clients.write().get(board_id).unwrap().status.lock() = status.data.status;
        } else if let Some(board_id) = packet.topic.strip_prefix("/sdcp/response/") {
            println!("Got command response from `{}`", board_id);
            println!("{:?}", String::from_utf8_lossy(&packet.data));
        }

        Ok(())
    }

    fn on_publish_ack(&self, client_id: u64, packet: PublishAckPacket) -> Result<()> {
        println!(
            "Client `{client_id}` acknowledged packet `{}`",
            packet.packet_id
        );
        Ok(())
    }

    fn on_disconnect(&self, client_id: u64) -> Result<()> {
        let machine_id = self.client_ids.write().remove(&client_id);
        if let Some(machine_id) = machine_id {
            self.clients.write().remove(&machine_id);
            println!("Client `{machine_id}` disconnected");
        }
        Ok(())
    }
}

impl Mqtt {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MqttInner {
                server: Soon::empty(),
                clients: RwLock::new(HashMap::new()),
                client_ids: RwLock::new(HashMap::new()),
            }),
        }
    }

    pub fn send_command<Data: CommandTrait>(
        &self,
        mainboard_id: &str,
        command: Data,
    ) -> Result<()> {
        let clients = self.clients.read();
        let client = clients.get(mainboard_id).unwrap();
        let packet_id = client.next_id();

        let Some(client_id) = client.client_id else {
            eprintln!("Client `{mainboard_id}` is not connected. Command not sent.");
            return Ok(());
        };

        let data = Response {
            data: Command::new(
                Data::CMD,
                command,
                client.attributes.mainboard_id.to_owned(),
            ),
            id: client.machine_id.to_owned(),
        };
        let data = serde_json::to_vec(&data).unwrap();

        self.server
            .send_packet(
                client_id,
                PublishPacket {
                    flags: PublishFlags::QOS1,
                    topic: format!("/sdcp/request/{}", client.attributes.mainboard_id),
                    packet_id: Some(packet_id),
                    data,
                }
                .to_packet(),
            )
            .unwrap();

        Ok(())
    }

    pub fn add_future_client(&self, response: Response<FullStatusData>) {
        let mainboard_id = &response.data.attributes.mainboard_id;
        if self.clients.read().contains_key(mainboard_id) {
            println!("Client `{mainboard_id}` already exists.");
            return;
        }

        let mainboard_id = mainboard_id.clone();
        let client = MqttClient {
            attributes: response.data.attributes,
            status: Mutex::new(response.data.status),
            machine_id: response.id,
            client_id: None,
            next_packet_id: AtomicU16::new(0),
        };

        let mut clients = self.clients.write();
        clients.insert(mainboard_id, client);
    }
}

impl MqttClient {
    fn next_id(&self) -> u16 {
        self.next_packet_id.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for Mqtt {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Mqtt {
    type Target = MqttInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for Mqtt {
    fn drop(&mut self) {
        for mainboard_id in self.clients.read().keys() {
            println!("Disconnecting `{mainboard_id}`");
            let _ = self.send_command(mainboard_id, DisconnectCommand);
        }
    }
}

use nalgebra::Vector3;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use slicer::mesh::Mesh;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 4],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

pub struct RenderedMesh {
    pub mesh: Mesh,
    vertices: Vec<ModelVertex>,
    buffers: Option<RenderedMeshBuffers>,
}

pub struct RenderedMeshBuffers {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl RenderedMesh {
    pub fn from_mesh(mesh: Mesh) -> Self {
        let normals = mesh
            .faces
            .iter()
            .enumerate()
            .map(|(idx, face)| {
                let (p1, p2, p3) = (
                    mesh.vertices[face[0] as usize],
                    mesh.vertices[face[1] as usize],
                    mesh.vertices[face[2] as usize],
                );
                let a = p2 - p1;
                let b = p3 - p1;
                (idx, a.cross(&b).normalize())
            })
            .collect::<Vec<_>>();

        let mut vertex_faces = vec![Vec::new(); mesh.vertices.len()];
        for (face_idx, face) in mesh.faces.iter().enumerate() {
            for vertex_idx in face.iter() {
                vertex_faces[*vertex_idx as usize].push(face_idx);
            }
        }

        let vertices = mesh
            .vertices
            .iter()
            .enumerate()
            .map(|(idx, v)| ModelVertex {
                position: [v.x, v.y, v.z, 1.0],
                tex_coords: [0.0, 0.0],
                normal: {
                    let mut normal = Vector3::new(0.0, 0.0, 0.0);
                    for face_idx in &vertex_faces[idx] {
                        normal += normals[*face_idx].1;
                    }
                    normal.normalize().into()
                },
            })
            .collect::<Vec<ModelVertex>>();

        Self {
            mesh,
            vertices,
            buffers: None,
        }
    }

    pub fn try_get_buffers(&self) -> Option<&RenderedMeshBuffers> {
        self.buffers.as_ref()
    }

    pub fn take_buffers(&mut self) -> Option<RenderedMeshBuffers> {
        self.buffers.take()
    }

    pub fn get_buffers(&mut self, device: &Device) -> &RenderedMeshBuffers {
        if self.buffers.is_none() {
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.vertices),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            });

            let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.mesh.faces),
                usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            });

            self.buffers = Some(RenderedMeshBuffers {
                vertex_buffer,
                index_buffer,
            });
        }

        self.buffers.as_ref().unwrap()
    }
}

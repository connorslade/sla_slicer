use egui_wgpu::ScreenDescriptor;
use wgpu::{CommandEncoder, Device, Queue, RenderPass};

use super::WorkspaceRenderCallback;

pub mod build_plate;
pub mod model;

pub trait Pipeline {
    fn prepare(
        &self,
        device: &Device,
        queue: &Queue,
        screen_descriptor: &ScreenDescriptor,
        encoder: &mut CommandEncoder,
        resources: &WorkspaceRenderCallback,
    );
    fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>, resources: &WorkspaceRenderCallback);
}

#[macro_export]
macro_rules! include_shader {
    ($shader:literal) => {
        include_str!(concat!("../../shaders/", $shader))
    };
}

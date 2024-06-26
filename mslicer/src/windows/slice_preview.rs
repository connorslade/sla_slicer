use eframe::Frame;
use egui::{Context, DragValue, RichText, Sense, Slider, Vec2, Window};
use egui_wgpu::Callback;
use goo_format::LayerDecoder;
use nalgebra::Vector2;

use crate::{
    app::App, components::vec2_dragger, render::slice_preview::SlicePreviewRenderCallback,
};

pub fn ui(app: &mut App, ctx: &Context, _frame: &mut Frame) {
    if let Some(result) = app.slice_result.lock().unwrap().as_mut() {
        Window::new("Slice Preview")
            .resizable([true, true])
            .show(ctx, move |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        Slider::new(&mut result.slice_preview_layer, 1..=result.goo.layers.len())
                            .vertical()
                            .show_value(false),
                    );

                    result.slice_preview_layer =
                        result.slice_preview_layer.clamp(1, result.goo.layers.len());
                    let new_preview = if result.last_preview_layer != result.slice_preview_layer {
                        result.last_preview_layer = result.slice_preview_layer;
                        let (width, height) = (
                            result.goo.header.x_resolution as u32,
                            result.goo.header.y_resolution as u32,
                        );

                        let layer_data = &result.goo.layers[result.slice_preview_layer - 1].data;
                        let decoder = LayerDecoder::new(layer_data);

                        let mut image = vec![0; (width * height) as usize];
                        let mut pixel = 0;
                        for run in decoder {
                            for _ in 0..run.length {
                                image[pixel] = run.value;
                                pixel += 1;
                            }
                        }

                        Some(image)
                    } else {
                        None
                    };

                    result.preview_scale = result.preview_scale.max(0.1);
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        let available_size = ui.available_size();
                        let (rect, _response) = ui.allocate_exact_size(
                            Vec2::new(
                                available_size.x,
                                available_size.x / result.goo.header.x_resolution as f32
                                    * result.goo.header.y_resolution as f32,
                            ),
                            Sense::drag(),
                        );
                        let callback = Callback::new_paint_callback(
                            rect,
                            SlicePreviewRenderCallback {
                                dimensions: Vector2::new(
                                    result.goo.header.x_resolution as u32,
                                    result.goo.header.y_resolution as u32,
                                ),
                                offset: result.preview_offset,
                                scale: result.preview_scale,
                                new_preview,
                            },
                        );
                        ui.painter().add(callback);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut result.slice_preview_layer)
                            .clamp_range(1..=result.goo.layers.len())
                            .custom_formatter(|n, _| format!("{}/{}", n, result.goo.layers.len())),
                    );
                    result.slice_preview_layer +=
                        ui.button(RichText::new("+").monospace()).clicked() as usize;
                    result.slice_preview_layer -=
                        ui.button(RichText::new("-").monospace()).clicked() as usize;

                    ui.separator();
                    ui.label("Offset");
                    vec2_dragger(ui, result.preview_offset.as_mut(), |x| x);

                    ui.separator();
                    ui.label("Scale");
                    ui.add(DragValue::new(&mut result.preview_scale));
                });
            });
    }
}

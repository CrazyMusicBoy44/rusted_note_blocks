use crate::AudioSession;
use rnb_core::{
    notes::{InstrumentIndex, Note},
    song::Song,
};
use std::collections::HashMap;

#[derive()]
pub struct RNBUi {
    scene_rect: egui::Rect,
    // scene_rect_bounds: egui::Rect,
    song: Song,
    texture_cache: TextureCache,
}

impl RNBUi {
    pub fn new(_ctx: &egui::Context, song: Song) -> Self {
        Self {
            scene_rect: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(30.0, 30.0)),
            // scene_rect_bounds: egui::Rect::from_min_max(egui::pos2(0.0, 0.0),egui::pos2(30.0, 30.0),),
            song,
            texture_cache: TextureCache::default(),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        audio_constructor: impl Fn(&Song) -> Box<dyn AudioSession>,
    ) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Scene::new()
                .zoom_range(4.0..=1024.0)
                // .scene_rect_bounds(self.scene_rect_bounds)
                .drag_pan_buttons(egui::DragPanButtons::empty())
                .show(ui, &mut self.scene_rect, |ui| {
                    let clip_rect = ui.clip_rect();
                    let start_tick = clip_rect.min.x.max(0.0).floor() as u32;
                    let end_tick = clip_rect.max.x.ceil() as u32;
                    let start_layer = clip_rect.min.y.max(0.0).floor() as u32;
                    let end_layer = clip_rect.max.y.ceil() as u32;

                    for tick in start_tick..end_tick {
                        for layer in start_layer..end_layer {
                            Self::try_render_note_at_pos(
                                &self.song,
                                &mut self.texture_cache,
                                tick,
                                layer,
                                ui,
                            );
                        }
                    }
                })
        });
    }

    fn try_render_note_at_pos(
        song: &Song,
        texture_cache: &mut TextureCache,
        x: u32,
        y: u32,
        ui: &egui::Ui,
    ) {
        if let Some(note) = song.get_note_at(x, y)
            && let Some(texture) = texture_cache.get_texture_from_note(&note, song, ui)
        {
            egui::Image::new(&texture).paint_at(
                ui,
                egui::Rect::from_min_max(
                    egui::pos2(x as f32, y as f32),
                    egui::pos2((x + 1) as f32, (y + 1) as f32),
                ),
            )
        };
    }
}

#[derive(Default)]
struct TextureCache {
    textures: HashMap<InstrumentIndex, egui::TextureHandle>,
}

impl TextureCache {
    fn get_texture_from_note(
        &mut self,
        note: &Note,
        song: &Song,
        ui: &egui::Ui,
    ) -> Option<egui::TextureHandle> {
        self.get_texture(note.instrument_index, song, ui)
    }

    fn get_texture(
        &mut self,
        instrument_index: InstrumentIndex,
        song: &Song,
        ui: &egui::Ui,
    ) -> Option<egui::TextureHandle> {
        if let Some(texture) = self.textures.get(&instrument_index) {
            return Some(texture.clone());
        };
        if let Some(instrument) = song.instruments.get_instrument(instrument_index) {
            let texture = ui.ctx().load_texture(
                instrument.name.clone(),
                png_bytes_to_color_image(&instrument.raw_image.raw_image).ok()?,
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Linear,
                    ..Default::default()
                },
            );
            self.textures.insert(instrument_index, texture.clone());
            return Some(texture);
        };
        None
    }
}

fn png_bytes_to_color_image(png_bytes: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png)?;
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

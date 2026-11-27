use crate::RNBPlatform;
use rnb_audio::Mixer;
use rnb_core::{GridPoint, GridRect, InstrumentIndex, Instruments, Note, Song};
use std::collections::HashMap;

#[derive()]
pub struct RNBGui<P: RNBPlatform> {
    scene_rect: egui::Rect,
    // scene_rect_bounds: egui::Rect,
    song: Song,
    texture_cache: TextureCache,
    _platform: Option<P>,
    _mixer: Option<Mixer>,
}

impl<P: RNBPlatform> RNBGui<P> {
    #[must_use]
    pub fn new(_ctx: &egui::Context, song: Song) -> Self {
        Self {
            scene_rect: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(30.0, 30.0)),
            // scene_rect_bounds: egui::Rect::from_min_max(egui::pos2(0.0, 0.0),egui::pos2(30.0, 30.0),),
            song,
            texture_cache: TextureCache::default(),
            _platform: None,
            _mixer: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _platform: P) {
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

                    for (point, note) in self.song.notes.notes_between(GridRect::from_points((start_tick, start_layer).into(), (end_tick, end_layer).into())) {
                        Self::try_render_note_at_pos(&self.song.instruments, note, &mut self.texture_cache, point, ui);
                    }
                })
        });
    }

    fn try_render_note_at_pos(
        instruments: &Instruments,
        note: Note,
        texture_cache: &mut TextureCache,
        point: GridPoint,
        ui: &egui::Ui,
    ) {
        if let Some(texture) = texture_cache.get_texture_from_note(&note, instruments, ui) {
            egui::Image::new(&texture).paint_at(
                ui,
                egui::Rect::from_min_max(
                    egui::pos2(point.x as f32, point.y as f32),
                    egui::pos2((point.x + 1) as f32, (point.y + 1) as f32),
                ),
            );
        }
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
        instruments: &Instruments,
        ui: &egui::Ui,
    ) -> Option<egui::TextureHandle> {
        self.get_texture(note.instrument_index, instruments, ui)
    }

    fn get_texture(
        &mut self,
        instrument_index: InstrumentIndex,
        instruments: &Instruments,
        ui: &egui::Ui,
    ) -> Option<egui::TextureHandle> {
        if let Some(texture) = self.textures.get(&instrument_index) {
            return Some(texture.clone());
        }
        if let Some(instrument) = instruments.get_instrument(instrument_index) {
            let texture = ui.ctx().load_texture(
                instrument.name.clone(),
                png_bytes_to_color_image(instrument.raw_image.as_byte_ref()).ok()?,
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Linear,
                    ..Default::default()
                },
            );
            self.textures.insert(instrument_index, texture.clone());
            return Some(texture);
        }
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

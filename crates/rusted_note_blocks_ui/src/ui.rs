use rusted_note_blocks_core::{load, song::Song, notes::{Note, InstrumentIndex}};
use std::collections::HashMap;

#[derive()]
pub struct RustedNoteBlocksApp {
    scene_rect: egui::Rect,
    scene_rect_bounds: egui::Rect,
    song: Song,
    texture_cache: TextureCache,
}

impl RustedNoteBlocksApp {
    pub fn new(_ctx: &egui::Context) -> Self {
        Self {
            scene_rect: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(30.0, 30.0)),
            scene_rect_bounds: egui::Rect::from_min_max(
                egui::pos2(0.0, 0.0),
                egui::pos2(30.0, 30.0),
            ),
            song: load::load_from_nbs(include_bytes!(
            "C:\\Users\\Jesse\\AppData\\Local\\Minecraft Note Block Studio\\Songs\\mossplume_marsh.nbs",
        )).unwrap_or_default(),
        texture_cache: TextureCache::default(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Scene::new()
                .zoom_range(0.1..=1024.0)
                // .scene_rect_bounds(self.scene_rect_bounds)
                .show(ui, &mut self.scene_rect, |ui| {
                    ui.painter().rect_filled(
                        self.scene_rect_bounds,
                        egui::CornerRadius::from(1),
                        egui::Color32::PLACEHOLDER,
                    );
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::CornerRadius::ZERO,
                        egui::Color32::RED,
                    );
                    ui.label("test");
                    ui.label("test2");
                    if let Some(note) = self.song.get_note_at((0,0)) 
                        && let Some(texture) = self.texture_cache.get_texture_from_note(&note, &self.song, ui) {
                            ui.add(
                                egui::Image::new(&texture)
                            );
                    };
                })
        });
    }
}

#[derive(Default)]
struct TextureCache {
    textures: HashMap<InstrumentIndex, egui::TextureHandle>
}

impl TextureCache {
    fn get_texture_from_note(&mut self, note: &Note, song: &Song, ui: &egui::Ui) -> Option<egui::TextureHandle> {
        self.get_texture(note.instrument_index, song, ui)
    }

    fn get_texture(&mut self, instrument_index: InstrumentIndex, song: &Song, ui: &egui::Ui) -> Option<egui::TextureHandle> {
        if let Some(texture) = self.textures.get(&instrument_index) {
            return Some(texture.clone());
        };
        if let Some(instrument) = song.instruments.get_instrument(instrument_index) {
            let texture = ui.ctx().load_texture(
                instrument.name.clone(),
                png_bytes_to_color_image(&instrument.raw_image).ok()?,
                egui::TextureOptions { magnification: egui::TextureFilter::Nearest, minification: egui::TextureFilter::Linear, ..Default::default()}
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


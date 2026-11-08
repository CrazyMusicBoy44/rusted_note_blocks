use rusted_note_blocks_core::{load, song::Song};
pub struct RustedNoteBlocksApp {
    scene_rect: egui::Rect,
    scene_rect_bounds: egui::Rect,
    _song: Song,
}

impl RustedNoteBlocksApp {
    pub fn new(_ctx: &egui::Context) -> Self {
        let _song: Song = load::load_from_nbs(String::from(
            "C:\\Users\\Jesse\\AppData\\Local\\Minecraft Note Block Studio\\Songs\\mossplume_marsh.nbs",
        )).unwrap_or_default();
        Self {
            scene_rect: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(30.0, 30.0)),
            scene_rect_bounds: egui::Rect::from_min_max(
                egui::pos2(0.0, 0.0),
                egui::pos2(30.0, 30.0),
            ),
            _song,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Scene::new()
                .zoom_range(0.1..=1024.0)
                .scene_rect_bounds(self.scene_rect_bounds)
                .show(ui, &mut self.scene_rect, |ui| {
                    ui.painter().rect_filled(self.scene_rect_bounds, egui::CornerRadius::from(1), egui::Color32::PLACEHOLDER);
                    ui.painter().rect_filled(egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::CornerRadius::ZERO, egui::Color32::RED);
                    ui.label("test");
                    ui.label("test2");
                })
        });
    }
}

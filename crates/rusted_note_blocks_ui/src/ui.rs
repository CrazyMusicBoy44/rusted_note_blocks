use rusted_note_blocks_core::song::Song;
pub struct RustedNoteBlocksApp {
    scene_rect: egui::Rect,
    song: Song,
}

impl RustedNoteBlocksApp {
    pub fn new(_ctx: &egui::Context) -> Self {
        Self {
            scene_rect: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(30.0, 30.0)),
            ),
            song: Song::default(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Scene::new()
                .zoom_range(0.1..=100.0)
                .show(ui, &mut self.scene_rect, |ui| {
                    ui.label("Hello World");
                })
        });
    }
}

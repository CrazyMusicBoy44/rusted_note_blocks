use rusted_note_blocks_core::song::Song;
pub struct RustedNoteBlocksApp {
    viewport: egui::Rect,
    song: Song,
}

impl RustedNoteBlocksApp {
    pub fn new(_ctx: &egui::Context) -> Self {
        Self {
            viewport: egui::Rect::from_min_max(
                egui::pos2(-500.0, -500.0),
                egui::pos2(500.0, 500.0),
            ),
            song: Song::default(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Scene::new()
                .zoom_range(0.1..=100.0)
                .show(ui, &mut self.viewport, |ui| {
                    ui.label("Hello World");
                })
        });
    }
}

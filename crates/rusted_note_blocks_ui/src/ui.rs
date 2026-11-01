pub struct RustedNoteBlocksApp {
    scene_state: egui::Rect,
}

impl RustedNoteBlocksApp {
    pub fn new(_ctx: &egui::Context) -> Self {
        Self {
            scene_state: egui::Rect::from_min_max(
                egui::pos2(-500.0, -500.0),
                egui::pos2(500.0, 500.0),
            ),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Scene::new()
                .zoom_range(0.1..=100.0)
                .show(ui, &mut self.scene_state, |ui| {
                    ui.label("Hello World");
                })
        });
    }
}

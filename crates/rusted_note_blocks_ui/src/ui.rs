pub struct RustedNoteBlocksApp {

}

impl RustedNoteBlocksApp {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Hello World");
        });
    }
}

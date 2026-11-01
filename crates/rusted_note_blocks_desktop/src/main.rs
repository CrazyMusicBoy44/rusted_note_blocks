use rusted_note_blocks_ui::ui;

fn main() -> eframe::Result {
    eframe::run_native("Rusted Note Blocks Desktop", eframe::NativeOptions::default(), Box::new(|cc| Ok(Box::new(RustedNoteBlocksDesktop::new(cc)))))
}

struct RustedNoteBlocksDesktop {
    ui: ui::RustedNoteBlocksApp,
}

impl RustedNoteBlocksDesktop {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { ui: ui::RustedNoteBlocksApp { } }
    }
}

impl eframe::App for RustedNoteBlocksDesktop {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        ui::RustedNoteBlocksApp::ui(&mut self.ui, ui);
    }
}

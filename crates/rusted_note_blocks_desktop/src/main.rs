use rusted_note_blocks_core::load;
use rusted_note_blocks_ui::ui;

fn main() -> eframe::Result {
    test_song_loading();

    eframe::run_native(
        "Rusted Note Blocks Desktop",
        eframe::NativeOptions::default(),
        Box::new(|creation_context| {
            Ok(Box::new(RustedNoteBlocksDesktopApp::new(creation_context)))
        }),
    )
}

struct RustedNoteBlocksDesktopApp {
    ui: ui::RustedNoteBlocksApp,
}

impl RustedNoteBlocksDesktopApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            ui: ui::RustedNoteBlocksApp::new(&cc.egui_ctx),
        }
    }
}

impl eframe::App for RustedNoteBlocksDesktopApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        ui::RustedNoteBlocksApp::ui(&mut self.ui, ui);
    }
}

fn test_song_loading() {
    let testing_song = load::load_from_nbs(include_bytes!(
        "C:\\Users\\Jesse\\AppData\\Local\\Minecraft Note Block Studio\\Songs\\mossplume_marsh.nbs"
    ))
    .unwrap();
    println!("{}", testing_song.metadata.name);
}

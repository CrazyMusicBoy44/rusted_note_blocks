use rnb_core::load_from_bytes;
use rnb_gui::RNBGui;
use crate::platform::DesktopPlatform;

pub struct RNBDesktopApp {
    app_ui: RNBGui<DesktopPlatform>,
}

impl RNBDesktopApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Option<Self> {
        let file = rfd::FileDialog::new()
            .pick_file()
            .and_then(|path| std::fs::read(path).ok());
        let song_bytes = load_from_bytes(&file?).ok()?;

        Some(Self {
            app_ui: RNBGui::new(&cc.egui_ctx, song_bytes),
        })
    }
}

impl eframe::App for RNBDesktopApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        self.app_ui.show(ui, DesktopPlatform {});
    }
}

pub fn run() -> eframe::Result {
    eframe::run_native(
        "Rusted Note Blocks Desktop",
        eframe::NativeOptions::default(),
        Box::new(|creation_context| {
            Ok(Box::new(
                RNBDesktopApp::new(creation_context).expect("did not recieve a valid song"),
            ))
        }),
    )
}

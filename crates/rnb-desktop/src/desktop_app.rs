use rnb_core::load;
use rnb_ui::RNBUi;

pub(crate) struct RNBDesktopApp {
    app_ui: RNBUi,
}

impl RNBDesktopApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Option<Self> {
        let file = rfd::FileDialog::new()
            .pick_file()
            .and_then(|path| std::fs::read(path).ok());
        let song_bytes = load::load_from_nbs(&file?).ok()?;

        Some(Self {
            app_ui: RNBUi::new(&cc.egui_ctx, song_bytes),
        })
    }
}

impl eframe::App for RNBDesktopApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        self.app_ui.show(ui, |_| todo!());
    }
}

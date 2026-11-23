use rnb_core::load;
use rnb_ui::RNBUi;

fn main() -> eframe::Result {
    let _audio_test_device = test_audio();

    eframe::run_native(
        "Rusted Note Blocks Desktop",
        eframe::NativeOptions::default(),
        Box::new(|creation_context| {
            Ok(Box::new(
                RNBDesktopApp::new(creation_context).unwrap(),
            ))
        }),
    )
}

struct RNBDesktopApp {
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
        self.app_ui.show(ui);
    }
}

fn test_audio() -> tinyaudio::OutputDevice {
    let params = tinyaudio::OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 48000,
        channel_sample_count: 4096,
    };

    let mut phase = 0.0_f32;
    let freq = 440.0;
    let volume = 0.1;

    tinyaudio::run_output_device(params, move |samples: &mut [f32]| {
        for frame in samples.chunks_exact_mut(params.channels_count) {
            let sample = if phase < 0.5 { volume } else { -volume };
            frame.fill(sample);
            phase = (phase + freq / params.sample_rate as f32) % 1.0;
        }
    }).expect("i expect something here but it was error")
}

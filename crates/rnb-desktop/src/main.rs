mod desktop_app;
use desktop_app::RNBDesktopApp;
mod audio;
use audio::test_audio;

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

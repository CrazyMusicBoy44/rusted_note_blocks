mod desktop_app;
use desktop_app::run;
mod audio;
use audio::test_audio;
mod platform;

fn main() -> eframe::Result {
    // let _audio_test_device = test_audio();

    run()
}

mod desktop_app;
use desktop_app::run;
mod audio;
mod platform;

fn main() -> eframe::Result {
    run()
}

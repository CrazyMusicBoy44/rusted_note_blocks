#[derive(Clone, Debug)]
pub struct Instrument {
    name: String,
    pitch_center: f32,
    raw_image: Vec<u8>,
}
macro_rules! include_instrument {
    ($name:expr, $pitch:expr, $file:expr) => {
        Instrument {
            name: String::from($name),
            pitch_center: $pitch,
            raw_image: Vec::from(include_bytes!(concat!(
                "../../../assets/vanilla_instruments/",
                $file,
                ".png"
            ))),
        }
    };
}

fn vanilla_instruments() -> Vec<Instrument> {
    vec![
        include_instrument!("Harp", 0.0, "harp"),
        include_instrument!("Bass", 0.0, "bass"),
        include_instrument!("Bass Drum", 0.0, "bass_drum"),
        include_instrument!("Snare Drum", 0.0, "snare"),
        include_instrument!("Click", 0.0, "click"),
    ]
}

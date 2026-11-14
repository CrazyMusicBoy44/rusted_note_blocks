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

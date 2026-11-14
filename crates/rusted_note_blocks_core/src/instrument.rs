#[derive(Clone, Debug)]
pub struct Instrument {
    name: String,
    pitch_center: f32,
    raw_image: Vec<u8>,
}

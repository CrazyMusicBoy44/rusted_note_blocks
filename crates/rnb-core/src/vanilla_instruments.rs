use crate::{Instrument, RawImage, RawSample};

macro_rules! include_vanilla_instrument {
    ($name:expr, $pitch:expr, $file:expr) => {
        Instrument {
            name: String::from($name),
            pitch_center: $pitch,
            raw_image: RawImage::from(
                &include_bytes!(concat!("../../../assets/textures/vanilla/", $file, ".png"))[..],
            ),
            raw_sample: RawSample::from(
                &include_bytes!(concat!("../../../assets/samples/vanilla/", $file, ".ogg"))[..],
            )
        }
    };
}

pub fn vanilla_instruments() -> Vec<Instrument> {
    vec![
        include_vanilla_instrument!("Harp", 0.0, "harp"),
        include_vanilla_instrument!("Bass", 0.0, "bass"),
        include_vanilla_instrument!("Bass Drum", 0.0, "bass_drum"),
        include_vanilla_instrument!("Snare Drum", 0.0, "snare"),
        include_vanilla_instrument!("Click", 0.0, "click"),
        include_vanilla_instrument!("Guitar", 0.0, "guitar"),
        include_vanilla_instrument!("Flute", 0.0, "flute"),
        include_vanilla_instrument!("Bell", 0.0, "bell"),
        include_vanilla_instrument!("Chime", 0.0, "chime"),
        include_vanilla_instrument!("Xylophone", 0.0, "xylophone"),
        include_vanilla_instrument!("Iron Xylophone", 0.0, "iron_xylophone"),
        include_vanilla_instrument!("Cow Bell", 0.0, "cow_bell"),
        include_vanilla_instrument!("Didgeridoo", 0.0, "didgeridoo"),
        include_vanilla_instrument!("Bit", 0.0, "bit"),
        include_vanilla_instrument!("Banjo", 0.0, "banjo"),
        include_vanilla_instrument!("Pling", 0.0, "pling"),
        // include_vanilla_instrument!("Trumpet", 0.0, "trumpet"),
        // include_vanilla_instrument!("Exposed Trumpet", 0.0, "exposed_trumpet"),
        // include_vanilla_instrument!("Weathered Trumpet", 0.0, "weathered_trumpet"),
        // include_vanilla_instrument!("Oxidized Trumpet", 0.0, "oxidized_trumpet"),
    ]
}

use crate::notes::{InstrumentIndex};

#[derive(Clone, Debug)]
pub struct Instrument {
    name: String,
    pitch_center: f32,
    raw_image: Vec<u8>,
}

impl Instrument {}

#[derive(Clone, Debug)]
pub struct Instruments {
    vanilla_instruments: Vec<Instrument>,
    custom_instruments: Vec<Instrument>,
}

impl Default for Instruments {
    fn default() -> Self {
        Self::new()
    }
}

impl Instruments {
    fn new() -> Self {
        Self {
            vanilla_instruments: vanilla_instruments(),
            custom_instruments: Vec::new(),
        }
    }

    fn from_custom_instruments() -> Self {
        Self {
            vanilla_instruments: vanilla_instruments(),
            custom_instruments: Vec::new(),
        }
    }

    fn get_instrument(&self, instrument_index: InstrumentIndex) -> Option<Instrument> {
        match instrument_index {
            InstrumentIndex::Vanilla { .. } => self.vanilla_instruments.get(instrument_index.to_usize()).cloned(),
            InstrumentIndex::Custom { .. } => self.custom_instruments.get(instrument_index.to_usize()).cloned(),
        }
    }
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

use crate::{InstrumentIndex, Note, vanilla_instruments::vanilla_instruments};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Instrument {
    pub name: String,
    pub pitch_center: f32,
    pub raw_image: RawImage,
}

#[derive(Clone, Debug)]
pub struct RawImage {
    raw_image: Cow<'static, [u8]>,
}

impl RawImage {
    pub fn as_byte_ref(&self) -> &[u8] {
        &self.raw_image
    }
}

impl Default for RawImage {
    fn default() -> Self {
        Self {
            raw_image: Cow::Borrowed(include_bytes!("../../../assets/textures/note_block.png")),
        }
    }
}

impl From<&'static [u8]> for RawImage {
    fn from(value: &'static [u8]) -> Self {
        Self { raw_image: Cow::from(value), }
    }
}

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
    #[must_use]
    pub fn new() -> Self {
        Self {
            vanilla_instruments: vanilla_instruments(),
            custom_instruments: Vec::new(),
        }
    }

    #[must_use]
    pub fn from_custom_instruments(custom_instruments: Vec<Instrument>) -> Self {
        Self {
            vanilla_instruments: vanilla_instruments(),
            custom_instruments,
        }
    }

    #[must_use]
    pub fn get_instrument(&self, instrument_index: InstrumentIndex) -> Option<Instrument> {
        match instrument_index {
            InstrumentIndex::Vanilla { .. } => self
                .vanilla_instruments
                .get(instrument_index.to_usize())
                .cloned(),
            InstrumentIndex::Custom { .. } => self
                .custom_instruments
                .get(instrument_index.to_usize())
                .cloned(),
        }
    }

    #[must_use]
    pub fn get_instrument_from_note(&self, note: Note) -> Option<Instrument> {
        self.get_instrument(note.instrument_index)
    }
}

use crate::number_types::U0F16;
use crate::number_types::U8F8;
use std::num::NonZeroU16;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstrumentType {
    Vanilla { index: NonZeroU16 },
    Custom { index: NonZeroU16 },
}

impl Default for InstrumentType {
    fn default() -> InstrumentType {
        Self::Vanilla {
            index: NonZeroU16::MIN,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Note {
    pub instrument_type: InstrumentType,
    pub volume: U0F16,
    pub pitch: U8F8,
    pub pan: i8, // -100 to 100
}

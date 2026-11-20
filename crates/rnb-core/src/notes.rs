use crate::number_types::U0F16;
use crate::number_types::U8F8;
use std::num::NonZeroU16;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InstrumentIndex {
    Vanilla { index: NonZeroU16 },
    Custom { index: NonZeroU16 },
}

impl InstrumentIndex {
    pub(crate) fn to_usize(self) -> usize {
        match self {
            Self::Vanilla { index } | Self::Custom { index } => usize::from(index.get() - 1),
        }
    }
}

impl Default for InstrumentIndex {
    fn default() -> Self {
        Self::Vanilla {
            index: NonZeroU16::MIN,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Note {
    pub instrument_index: InstrumentIndex,
    pub volume: U0F16,
    pub pitch: U8F8,
    pub pan: i8, // -100 to 100
}

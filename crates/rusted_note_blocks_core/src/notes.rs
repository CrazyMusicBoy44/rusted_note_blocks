use crate::number_types::U8F8;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NoteType {
    Vanilla,
    Custom,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub note_type: NoteType,
    pub sample_index: u32,
    pub volume: u16, // Unsigned fixed point (0.16) number (from 0 to 1)
    pub pitch: U8F8,
}

impl Note {}

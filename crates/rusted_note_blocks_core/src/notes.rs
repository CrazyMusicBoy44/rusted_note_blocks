use crate::number_types::U0F16;
use crate::number_types::U8F8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NoteType {
    Vanilla,
    Custom,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Note {
    pub note_type: NoteType,
    pub sample_index: u32,
    pub volume: U0F16,
    pub pitch: U8F8,
}

impl Note {}

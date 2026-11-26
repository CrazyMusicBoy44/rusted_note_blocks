use crate::{Instruments, NoteGrid};

#[derive(Default, Clone, Debug)]
pub struct SongMetadata {
    pub name: String,
    pub author: String,
    pub original_author: String,
    pub description: String,
}

#[derive(Default, Clone, Debug)]
pub struct Song {
    pub metadata: SongMetadata,
    pub notes: NoteGrid,
    pub instruments: Instruments,
}

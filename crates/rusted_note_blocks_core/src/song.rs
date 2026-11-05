use crate::notes::Note;
use smallvec::SmallVec;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct SongMetadata {
    pub name: String,
}

#[derive(Default)]
pub struct Song {
    pub metadata: SongMetadata,
    pub notes: Vec<Vec<(i32, Note)>>,
}

impl Song {
    // pub fn get_note(&self, (tick, layer): (i32, i32)) -> Option<Note> {
    //     self.notes.get(&tick)?.get(layer as usize).copied()
    // }
}

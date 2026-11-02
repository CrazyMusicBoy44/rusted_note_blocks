use crate::notes::Note;
use std::collections::BTreeMap;
use smallvec::SmallVec;


#[derive(Default)]
pub struct SongMetadata {
    name: String,
}

#[derive(Default)]
pub struct Song {
    metadata: SongMetadata,
    notes: BTreeMap<i32, SmallVec<[Note; 32]>>
}

impl Song {
    pub fn get_note(&self, (tick, layer): (i32, i32)) -> Option<Note> {
        let maybe_column: Option<&SmallVec<[Note; 32]>> = self.notes.get(&tick);
        if let Some(column) = maybe_column {
            column.get(layer as usize).copied()
        } else {
            None
        }
    }
}

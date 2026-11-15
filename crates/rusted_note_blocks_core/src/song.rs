use crate::{notes::Note,instrument::{Instrument, Instruments}};

#[derive(Default, Clone, Debug)]
pub struct SongMetadata {
    pub name: String,
}

#[derive(Default, Clone, Debug)]
pub struct Song {
    pub metadata: SongMetadata,
    pub notes: Vec<Option<Tick>>,
    pub instruments: Instruments,
}

impl Song {
    pub fn get_note_at(&self, (x, y): (u32, u32)) -> Option<Note> {
        self.notes.get(x as usize)?.as_ref()?.get_note_at(y)
    }
}

#[derive(Default, Clone, Debug)]
pub struct Tick {
    notes: Vec<(u32, Note)>,
}

impl Tick {
    pub(crate) fn from_vec(mut vec: Vec<(u32, Note)>) -> Self {
        vec.sort_unstable_by_key(|(pos, _)| *pos);
        vec.dedup_by_key(|(pos, _)| *pos);
        Self { notes: vec }
    }
    pub fn get_note_at(&self, y: u32) -> Option<Note> {
        match self.notes.binary_search_by_key(&y, |(pos, _)| *pos) {
            Ok(index) => Some(self.notes[index].1),
            Err(_) => None,
        }
    }
    pub fn set_note_at(&mut self, y: u32, note: Note) {
        match self.notes.binary_search_by_key(&y, |(pos, _)| *pos) {
            Ok(index) => self.notes[index] = (y, note),
            Err(index) => self.notes.insert(index, (y, note)),
        }
    }
    pub fn get_notes_between(&self, min: u32, max: u32) -> &[(u32, Note)] {
        let start = self.notes.partition_point(|&(k, _)| k < min);
        let end = start + self.notes[start..].partition_point(|&(k, _)| k <= max);
        &self.notes[start..end]
    }
}

use crate::{instrument::Instruments, notes::Note};

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

#[derive(Default, Clone, Debug)]
pub struct NoteGrid {
    notes: Vec<Option<Tick>>,
}

impl NoteGrid {
    #[must_use]
    pub fn get_note_at(&self, x: u32, y: u32) -> Option<Note> {
        self.notes.get(x as usize)?.as_ref()?.get_note_at(y)
    }
}

impl From<Vec<Option<Tick>>> for NoteGrid {
    fn from(notes: Vec<Option<Tick>>) -> Self {
        Self {
            notes,
        }
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
    #[must_use]
    pub fn get_note_at(&self, y: u32) -> Option<Note> {
        self.notes
            .binary_search_by_key(&y, |(pos, _)| *pos)
            .map_or(None, |index| Some(self.notes[index].1))
    }
    pub fn set_note_at(&mut self, y: u32, note: Note) {
        match self.notes.binary_search_by_key(&y, |(pos, _)| *pos) {
            Ok(index) => self.notes[index] = (y, note),
            Err(index) => self.notes.insert(index, (y, note)),
        }
    }
    #[must_use]
    pub fn get_notes_between(&self, min: u32, max: u32) -> &[(u32, Note)] {
        let start = self.notes.partition_point(|&(k, _)| k < min);
        let end = start + self.notes[start..].partition_point(|&(k, _)| k <= max);
        &self.notes[start..end]
    }
}

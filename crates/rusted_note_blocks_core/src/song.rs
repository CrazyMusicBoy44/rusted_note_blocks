use crate::notes::Note;

#[derive(Default, Clone, Debug)]
pub struct SongMetadata {
    pub name: String,
}

#[derive(Default, Clone, Debug)]
pub struct Song {
    pub metadata: SongMetadata,
    pub notes: Vec<Option<Tick>>,
}

impl Song {}

#[derive(Default, Clone, Debug)]
pub struct Tick {
    notes: Vec<(u32, Note)>,
}

impl Tick {
    pub fn from_vec(mut vec: Vec<(u32, Note)>) -> Self {
        vec.sort_unstable_by_key(|(pos, _)| *pos);
        vec.dedup_by_key(|(pos, _)| *pos);
        Self { notes: vec }
    }
    pub fn get_note_at(&self, pos: u32) -> Option<Note> {
        match self.notes.binary_search_by_key(&pos, |(pos, _)| *pos) {
            Ok(index) => Some(self.notes.get(index)?.1),
            Err(_) => None,
        }
    }
    pub fn set_note_at(&mut self, pos: u32, note: Note) {
        match self.notes.binary_search_by_key(&pos, |(pos, _)| *pos) {
            Ok(index) => self.notes[index] = (pos, note),
            Err(index) => self.notes.insert(index, (pos, note)),
        }
    }
    pub fn get_notes_between(&self, min: u32, max: u32) -> &[(u32, Note)] {
        let start = self.notes.partition_point(|&(k, _)| k < min);
        let end = start + self.notes[start..].partition_point(|&(k, _)| k <= max);
        &self.notes[start..end]
    }
}

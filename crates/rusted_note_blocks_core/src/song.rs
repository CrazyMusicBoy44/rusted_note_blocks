use crate::notes::Note;
use std::collections::HashMap;

pub struct Chunk {
    notes: [[Option<Note>; 16]; 16], // Each inner array is a column of notes
}

pub struct SongMetadata {
    name: String,
}

pub struct Song {
    metadata: SongMetadata,
    chunks: HashMap<i32, HashMap<i32, Chunk>>, // Each inner array is a column of chunks
}

impl Song {
    pub fn add_chunk(&mut self, (x, y): (i32, i32), chunk: Chunk) {
        let column = self.chunks.entry(x).or_default();
        column.insert(y, chunk);
    }
}

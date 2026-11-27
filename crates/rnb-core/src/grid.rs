use crate::{Note, ToUsize};

#[derive(Default, Clone, Debug)]
pub struct NoteGrid {
    notes: Vec<Option<Tick>>,
}

impl NoteGrid {
    #[must_use]
    pub fn get_note_at(&self, point: GridPoint) -> Option<Note> {
        self.notes
            .get(point.x.to_usize())?
            .as_ref()?
            .get_note_at(point.y)
    }
}

impl NoteGrid {
    #[must_use]
    pub fn notes_between(&self, rect: GridRect) -> Vec<(GridPoint, Note)> {
        let len = self.notes.len();
        let start = rect.min.x.to_usize().min(len);
        let end = rect.max.x.to_usize().min(len);

        let mut note_list: Vec<(GridPoint, Note)> = Vec::new();

        for potential_tick_index in start..=end {
            if let Some(Some(tick)) = self.notes.get(potential_tick_index) {
                for &(y, note) in tick.notes_between(rect.min.y, rect.max.y) {
                    if let Ok(x) = u32::try_from(potential_tick_index) {
                        note_list.push((GridPoint { x, y }, note));
                    }
                }
            }
        }
        note_list
    }
}

impl From<Vec<Option<Tick>>> for NoteGrid {
    fn from(notes: Vec<Option<Tick>>) -> Self {
        Self { notes }
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
            .map_or(None, |index| Some(self.notes.get(index)?.1))
    }
    pub fn set_note_at(&mut self, y: u32, note: Note) {
        match self.notes.binary_search_by_key(&y, |(pos, _)| *pos) {
            Ok(index) => {
                if let Some(pair_ref) = self.notes.get_mut(index) {
                    *pair_ref = (y, note);
                }
            },
            Err(index) => self.notes.insert(index, (y, note)),
        }
    }
    #[must_use]
    fn notes_between(&self, min: u32, max: u32) -> &[(u32, Note)] {
        let start = self.notes.partition_point(|&(k, _)| k < min);
        let end = start.saturating_add(self.notes.get(start..).unwrap_or_default().partition_point(|&(k, _)| k <= max));
        self.notes.get(start..end).unwrap_or_default()
    }
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct GridPoint {
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for GridPoint {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct GridRect {
    min: GridPoint,
    max: GridPoint,
}

impl GridRect {
    #[must_use]
    pub fn from_points(point1: GridPoint, point2: GridPoint) -> Self {
        Self {
            min: GridPoint {
                x: point1.x.min(point2.x),
                y: point1.y.min(point2.y),
            },
            max: GridPoint {
                x: point1.x.max(point2.x),
                y: point1.y.max(point2.y),
            },
        }
    }
}

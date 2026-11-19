use crate::{
    instrument::{Instruments, Instrument},
    notes::{InstrumentIndex, Note},
    number_types::{U0F16, U8F8},
    song::{Song, SongMetadata, Tick},
};
use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadErrorKind {
    Unsupported,
    InvalidData,
    UnexpectedEof,
}

#[derive(Debug)]
pub struct LoadError {
    pub kind: LoadErrorKind,
    pub message: String,
}

impl LoadError {
    pub fn new(kind: LoadErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LoadError {}

pub type LoadResult<T> = Result<T, LoadError>;

pub fn load_from_nbs(bytes: &[u8]) -> LoadResult<Song> {
    let mut index = 0;
    if nbs_number::<i16>(bytes, &mut index)? != 0 || nbs_number::<i8>(bytes, &mut index)? < 4 {
        return Err(LoadError::new(
            LoadErrorKind::Unsupported,
            "Only NBS version 4 and above is supported",
        ));
    }
    let signed_vanilla_instrument_count: i8 = nbs_number(bytes, &mut index)?;
    let vanilla_instrument_count: u8 = if signed_vanilla_instrument_count < 0 {
        return Err(LoadError::new(
            LoadErrorKind::InvalidData,
            "Vanilla instrument count is negative",
        ));
    } else {
        signed_vanilla_instrument_count as u8
    };
    let song_length: i16 = nbs_number(bytes, &mut index)?;
    let layer_count: i16 = nbs_number(bytes, &mut index)?;
    let song_name: String = nbs_string(bytes, &mut index)?;
    let _song_author: String = nbs_string(bytes, &mut index)?;
    let _song_original_author: String = nbs_string(bytes, &mut index)?;
    let _song_description: String = nbs_string(bytes, &mut index)?;
    let raw_song_tempo: i16 = nbs_number(bytes, &mut index)?;
    let _song_tempo: f32 = f32::from(raw_song_tempo) / 100.0;
    let _auto_saving: i8 = nbs_number(bytes, &mut index)?;
    let _auto_saving_interval: i8 = nbs_number(bytes, &mut index)?;
    let _time_signature: i8 = nbs_number(bytes, &mut index)?;
    let _minutes_spent: i32 = nbs_number(bytes, &mut index)?;
    let _left_clicks: i32 = nbs_number(bytes, &mut index)?;
    let _right_clicks: i32 = nbs_number(bytes, &mut index)?;
    let _note_blocks_added: i32 = nbs_number(bytes, &mut index)?;
    let _note_blocks_removed: i32 = nbs_number(bytes, &mut index)?;
    let _imported_file_name: String = nbs_string(bytes, &mut index)?;
    let _loop_enabled: i8 = nbs_number(bytes, &mut index)?;
    let _max_loop_count: i8 = nbs_number(bytes, &mut index)?;
    let _loop_start_tick: i16 = nbs_number(bytes, &mut index)?;

    let mut tick: i16 = -1;
    if song_length < 0 {
        return Err(LoadError::new(
            LoadErrorKind::InvalidData,
            "Song length is negative",
        ));
    }
    let mut grid: Vec<Option<Tick>> = vec![None; (song_length + 16) as usize];

    loop {
        match nbs_number(bytes, &mut index)? {
            0 => break,
            val => tick += val,
        }
        let mut layer: i16 = -1;
        let mut tick_notes: Vec<(u32, Note)> = Vec::with_capacity(16);
        loop {
            match nbs_number(bytes, &mut index)? {
                0 => break,
                val => layer += val,
            }
            let note: Note = NBSNote {
                instrument: nbs_number(bytes, &mut index)?,
                key: nbs_number(bytes, &mut index)?,
                velocity: nbs_number(bytes, &mut index)?,
                pan: nbs_number(bytes, &mut index)?,
                pitch: nbs_number(bytes, &mut index)?,
            }
            .to_note(vanilla_instrument_count);
            tick_notes.push((layer as u32, note));
        }
        grid[tick as usize] = Some(Tick::from_vec(tick_notes));
    }

    for _ in 0..layer_count {
        let _layer_name = nbs_string(bytes, &mut index)?;
        let _layer_locked: i8 = nbs_number(bytes, &mut index)?;
        let _layer_volume: i8 = nbs_number(bytes, &mut index)?;
        let _layer_pan: u8 = nbs_number(bytes, &mut index)?;
    }

    let custom_instrument_count: u8 = nbs_number(bytes, &mut index)?;

    let mut custom_instruments: Vec<Instrument> = Vec::with_capacity(custom_instrument_count as usize);

    for _ in 0..custom_instrument_count {
        let custom_instrument_name = nbs_string(bytes, &mut index)?;
        let _custom_instrument_sound_file = nbs_string(bytes, &mut index)?;
        let custom_instrument_key: i8 = nbs_number(bytes, &mut index)?;
        let _custom_instrument_should_press_piano_key: i8 = nbs_number(bytes, &mut index)?;

        let custom_instrument = Instrument {
            name: custom_instrument_name,
            pitch_center: custom_instrument_key as f32,
            raw_image: Default::default(),
        };
        custom_instruments.push(custom_instrument);
    }

    Ok(Song {
        metadata: SongMetadata { name: song_name },
        notes: grid,
        instruments: Instruments::from_custom_instruments(custom_instruments),
    })
}

struct NBSNote {
    instrument: u8,
    key: i8,
    velocity: i8,
    pan: u8,
    pitch: i16,
}

impl NBSNote {
    fn to_note(&self, vanilla_instrument_count: u8) -> Note {
        let instrument_index = if self.instrument <= vanilla_instrument_count {
            InstrumentIndex::Vanilla {
                index: NonZeroU16::new(u16::from(self.instrument) + 1).unwrap(),
            }
        } else {
            InstrumentIndex::Custom {
                index: NonZeroU16::new(u16::from(self.instrument - vanilla_instrument_count) + 1)
                    .unwrap(),
            }
        };

        Note {
            instrument_index,
            volume: U0F16::from_nbs_velocity(self.velocity),
            pitch: U8F8::from_key_and_pitch(self.key, self.pitch),
            pan: (i16::from(self.pan) - 100) as i8,
        }
    }
}

fn nbs_number<T: FromLittleEndianBytes>(bytes: &[u8], index: &mut usize) -> LoadResult<T> {
    let end = *index + T::SIZE;

    let slice = bytes.get(*index..end).ok_or_else(|| {
        LoadError::new(
            LoadErrorKind::UnexpectedEof,
            format!(
                "Unexpected end of file while parsing nbs number {}",
                std::any::type_name::<T>()
            ),
        )
    })?;

    let val = T::from_little_endian_slice(slice)?;

    *index = end;

    Ok(val)
}

fn nbs_string(bytes: &[u8], starting_index: &mut usize) -> LoadResult<String> {
    let len: i32 = nbs_number(bytes, starting_index)?;

    let len = usize::try_from(len).map_err(|_| {
        LoadError::new(LoadErrorKind::InvalidData, "NBS String length was negative")
    })?;

    let end = *starting_index + len;

    let slice = bytes.get(*starting_index..end).ok_or_else(|| {
        LoadError::new(
            LoadErrorKind::UnexpectedEof,
            "Unexpected end of file while reading NBS string",
        )
    })?;

    *starting_index = end;

    String::from_utf8(slice.to_vec())
        .map_err(|_| LoadError::new(LoadErrorKind::InvalidData, "NBS String data is not valid"))
}

trait FromLittleEndianBytes: Sized {
    const SIZE: usize = std::mem::size_of::<Self>();
    fn from_little_endian_slice(slice: &[u8]) -> LoadResult<Self>;
}

macro_rules! impl_from_little_endian_bytes {
    ($($t:ty),*) => {
        $(
            impl FromLittleEndianBytes for $t {
                fn from_little_endian_slice(slice: &[u8]) -> LoadResult<Self> {
                    let array: [u8; Self::SIZE] = slice
                        .try_into()
                        .map_err(|_| LoadError::new(LoadErrorKind::InvalidData, "Failed to parse bytes"))?;
                    Ok(<$t>::from_le_bytes(array))
                }
            }
        )*
    };
}

impl_from_little_endian_bytes!(i8, u8, i16, u16, i32, u32, i64, u64);

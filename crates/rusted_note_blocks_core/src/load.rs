use crate::{
    notes::{Note, NoteType},
    number_types::{U0F16, U8F8},
    song::{Song, SongMetadata},
    song::Tick
};
use std::{fs, io, num::NonZeroU16};

pub fn load_from_nbs(filename: String) -> io::Result<Song> {
    let file_contents = fs::read(filename)?;
    let mut index = 0;
    if nbs_number::<i16>(&file_contents, &mut index)? != 0
        || nbs_number::<i8>(&file_contents, &mut index)? < 4
    {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Only NBS version 4 and above is supported",
        ));
    }
    let signed_vanilla_instrument_count: i8 = nbs_number(&file_contents, &mut index)?;
    let vanilla_instrument_count: u8 = if signed_vanilla_instrument_count < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Vanilla instrument count is negative",
        ));
    } else {
        signed_vanilla_instrument_count as u8
    };
    let song_length: i16 = nbs_number(&file_contents, &mut index)?;
    let _layer_count: i16 = nbs_number(&file_contents, &mut index)?;
    let song_name: String = nbs_string(&file_contents, &mut index)?;
    let _song_author: String = nbs_string(&file_contents, &mut index)?;
    let _song_original_author: String = nbs_string(&file_contents, &mut index)?;
    let _song_description: String = nbs_string(&file_contents, &mut index)?;
    let raw_song_tempo: i16 = nbs_number(&file_contents, &mut index)?;
    let _song_tempo: f32 = raw_song_tempo as f32 / 100.0;
    let _auto_saving: i8 = nbs_number(&file_contents, &mut index)?;
    let _auto_saving_interval: i8 = nbs_number(&file_contents, &mut index)?;
    let _time_signature: i8 = nbs_number(&file_contents, &mut index)?;
    let _minutes_spent: i32 = nbs_number(&file_contents, &mut index)?;
    let _left_clicks: i32 = nbs_number(&file_contents, &mut index)?;
    let _right_clicks: i32 = nbs_number(&file_contents, &mut index)?;
    let _note_blocks_added: i32 = nbs_number(&file_contents, &mut index)?;
    let _note_blocks_removed: i32 = nbs_number(&file_contents, &mut index)?;
    let _imported_file_name: String = nbs_string(&file_contents, &mut index)?;
    let _loop_enabled: i8 = nbs_number(&file_contents, &mut index)?;
    let _max_loop_count: i8 = nbs_number(&file_contents, &mut index)?;
    let _loop_start_tick: i16 = nbs_number(&file_contents, &mut index)?;

    let mut tick: i16 = -1;
    if song_length < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Song length is negative",
        ));
    }
    let mut grid: Vec<Option<Tick>> = vec![None; (song_length + 16) as usize];

    loop {
        match nbs_number(&file_contents, &mut index)? {
            0 => break,
            val => tick += val,
        }
        let mut layer: i16 = -1;
        let mut tick_notes: Vec<(u32, Note)> = Vec::with_capacity(16);
        loop {
            match nbs_number(&file_contents, &mut index)? {
                0 => break,
                val => layer += val,
            }
            let note: Note = NBSNote {
                instrument: nbs_number(&file_contents, &mut index)?,
                key: nbs_number(&file_contents, &mut index)?,
                velocity: nbs_number(&file_contents, &mut index)?,
                pan: nbs_number(&file_contents, &mut index)?,
                pitch: nbs_number(&file_contents, &mut index)?,
            }
            .to_note(vanilla_instrument_count);
            tick_notes.push((layer as u32, note));
        }
        grid[tick as usize] = Some(Tick::from_vec(tick_notes));
    }

    Ok(Song {
        metadata: SongMetadata { name: song_name },
        notes: grid,
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
        let (note_type, instrument) = if self.instrument <= vanilla_instrument_count {
            (NoteType::Vanilla, self.instrument)
        } else {
            (NoteType::Custom, self.instrument - vanilla_instrument_count)
        };

        Note {
            note_type,
            sample_index: NonZeroU16::new(instrument as u16 + 1).unwrap(),
            volume: U0F16::from_nbs_velocity(self.velocity),
            pitch: U8F8::from_key_and_pitch(self.key, self.pitch),
            pan: (i16::from(self.pan) - 100) as i8,
        }
    }
}

fn nbs_number<T: FromLittleEndianBytes>(bytes: &[u8], starting_index: &mut usize) -> io::Result<T> {
    let end = *starting_index + T::SIZE;

    let slice = bytes.get(*starting_index..end).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!(
                "Unexpected end of file while parsing nbs number {}",
                std::any::type_name::<T>()
            ),
        )
    })?;

    let val = T::from_little_endian_slice(slice)?;

    *starting_index = end;

    Ok(val)
}

fn nbs_string(bytes: &[u8], starting_index: &mut usize) -> io::Result<String> {
    let len: i32 = nbs_number(bytes, starting_index)?;

    let len = usize::try_from(len).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "NBS String length was negative")
    })?;

    let end = *starting_index + len;

    let slice = bytes.get(*starting_index..end).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Unexpected end of file while reading NBS string",
        )
    })?;

    *starting_index = end;

    String::from_utf8(slice.to_vec())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "NBS String data is not valid"))
}

trait FromLittleEndianBytes: Sized {
    const SIZE: usize = std::mem::size_of::<Self>();
    fn from_little_endian_slice(slice: &[u8]) -> io::Result<Self>;
}

macro_rules! impl_from_little_endian_bytes {
    ($($t:ty),*) => {
        $(
            impl FromLittleEndianBytes for $t {
                fn from_little_endian_slice(slice: &[u8]) -> io::Result<Self> {
                    let array: [u8; Self::SIZE] = slice
                        .try_into()
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse bytes"))?;
                    Ok(<$t>::from_le_bytes(array))
                }
            }
        )*
    };
}

impl_from_little_endian_bytes!(i8, u8, i16, u16, i32, u32, i64, u64);

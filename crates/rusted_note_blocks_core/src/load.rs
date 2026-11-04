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

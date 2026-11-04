
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

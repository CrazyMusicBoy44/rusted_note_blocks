use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[must_use]
pub struct U8F8(pub u16);

impl U8F8 {
    #[must_use]
    pub fn to_f32(&self) -> f32 {
        f32::from(self.0) / 256.0
    }

    /// Returns a U8F8 based on raw NBS `key` and `pitch` values
    ///
    /// # Errors
    ///
    /// [`NumberError::WasNegative`] if the resulting key/pitch combo would be below 0 (only possible with negative
    /// pitch)
    /// [`NumberError::OutOfBounds`] if the pitch is abnormally large
    pub fn from_nbs_key_and_pitch(key: i8, pitch: i16) -> Result<Self, NumberError> {
        let expanded: u32 = (i32::from(key)
            .saturating_mul(100)
            .saturating_add(i32::from(pitch)))
        .try_into()
        .map_err(|_| NumberError::WasNegative)?;

        Ok(Self(
            u16::try_from(expanded.saturating_mul(256) / 100)
                .map_err(|_| NumberError::OutOfBounds)?,
        ))
    }
}

impl fmt::Debug for U8F8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.to_f32())
    }
}

impl From<u8> for U8F8 {
    fn from(n: u8) -> Self {
        Self(u16::from(n) * 256)
    }
}

impl From<f32> for U8F8 {
    fn from(n: f32) -> Self {
        Self((n * 256.0) as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[must_use]
pub struct U0F16(pub u16);

impl U0F16 {
    #[must_use]
    pub fn to_f32(&self) -> f32 {
        f32::from(self.0) / 65536.0
    }
    pub const fn from_nbs_velocity(velocity: i8) -> Self {
        Self(((velocity as u32 * 65535) / 100) as u16)
    }
    #[must_use]
    pub fn to_nbs_velocity(&self) -> i8 {
        ((u32::from(self.0) * 100) / 65535) as i8
    }
}

impl fmt::Debug for U0F16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.to_f32())
    }
}

impl From<f32> for U0F16 {
    fn from(n: f32) -> Self {
        Self((n * 65536.0) as u16)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NumberError {
    OutOfBounds,
    WasNegative,
    NotANumber,
}

impl Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, "{self:?}: Number was not in the valid range"),
            Self::WasNegative => write!(f, "{self:?}: Number was negative and shouldn't have"),
            Self::NotANumber => write!(f, "{self:?}: Number wasn't a number"),
        }
    }
}

impl Error for NumberError {}

#[cfg(target_pointer_width = "16")]
compile_error!("this crate requires a 32-bit or 64-bit target");

pub trait ToUsize {
    fn to_usize(self) -> usize;
}

impl ToUsize for u32 {
    #[inline]
    #[allow(clippy::as_conversions)]
    fn to_usize(self) -> usize {
        self as usize
    }
}

use std::{fmt, ops::{Add, Sub}};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[must_use]
pub struct U8F8(pub u16);

impl U8F8 {
    #[must_use]
    pub fn to_f32(&self) -> f32 {
        f32::from(self.0) / 256.0
    }
    pub fn from_key_and_pitch(key: i8, pitch: i16) -> Self {
        Self(((i32::from(key) * 100 + i32::from(pitch)) * 64 / 25) as u16)
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

impl Sub for U8F8 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Sub<u8> for U8F8 {
    type Output = Self;
    fn sub(self, other: u8) -> Self::Output {
        Self(self.0 - u16::from(other) * 256)
    }
}

impl Add for U8F8 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Add<u8> for U8F8 {
    type Output = Self;
    fn add(self, other: u8) -> Self::Output {
        Self(self.0 + u16::from(other) * 256)
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

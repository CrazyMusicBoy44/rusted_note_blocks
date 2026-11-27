use crate::numbers::NumberError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadErrorKind {
    Unsupported,
    InvalidData,
    UnexpectedEof,
    NumberOutOfBounds,
    NumberWasNegative,
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
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl From<NumberError> for LoadError {
    fn from(value: NumberError) -> Self {
        match value {
            NumberError::OutOfBounds => Self { kind: LoadErrorKind::NumberOutOfBounds, message: "Number was out of bounds".into() },
            NumberError::WasNegative => Self { kind: LoadErrorKind::NumberWasNegative, message: "Number should've been positive".into() },
        }
    }
}

impl std::error::Error for LoadError {}

pub type LoadResult<T> = Result<T, LoadError>;

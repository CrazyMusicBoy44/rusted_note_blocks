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
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for LoadError {}

pub type LoadResult<T> = Result<T, LoadError>;

mod nbs;
pub use nbs::load_from_bytes;

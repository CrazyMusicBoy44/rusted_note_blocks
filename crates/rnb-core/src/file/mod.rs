mod error;
pub use error::{LoadResult, LoadError, LoadErrorKind};

mod nbs;
pub use nbs::load_from_bytes;

mod error;
pub use error::{LoadError, LoadErrorKind, LoadResult};

mod nbs;
pub use nbs::load_from_bytes;

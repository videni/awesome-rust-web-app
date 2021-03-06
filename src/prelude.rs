use crate::error;
use std::result;

pub use crate::error::Error;

pub type Result<T, E = error::Error> = result::Result<T, E>;

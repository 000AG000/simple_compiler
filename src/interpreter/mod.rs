mod frame;
mod interpret;

pub use interpret::exec;
pub use crate::error::{GlobalError,ErrorKind,RuntimeErrorKind};
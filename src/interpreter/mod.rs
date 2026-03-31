mod frame;
mod interpret;

pub use crate::error::{ErrorKind, GlobalError, RuntimeErrorKind};
pub use interpret::exec;

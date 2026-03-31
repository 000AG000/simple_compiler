mod frame;
mod main_interpreter;

pub use crate::error::{ErrorKind, GlobalError, RuntimeErrorKind};
pub use main_interpreter::exec;

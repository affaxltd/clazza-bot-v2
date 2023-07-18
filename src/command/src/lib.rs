#![allow(clippy::must_use_candidate, clippy::missing_errors_doc)]

#[macro_use]
extern crate log;

pub use command_macro::*;

mod command;
pub use command::*;

pub mod utils;

pub mod prelude {
    pub use crate::utils::args::*;

    pub use crate::command::result::*;

    pub use crate::*;
}

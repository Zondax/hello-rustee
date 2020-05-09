mod client;
mod error;
pub mod os;
mod utils;

pub use {client::*, error::*, utils::*};

pub type Result<T> = core::result::Result<T, Error>;

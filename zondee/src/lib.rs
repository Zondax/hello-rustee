#![no_std]

mod array;
mod error;
mod stack_str;
mod stack_vec;
mod uuid;
mod utils;

pub use {array::*, error::*, stack_str::*, uuid::*, stack_vec::*, utils::*};

pub type Result<T> = core::result::Result<T, Error>;

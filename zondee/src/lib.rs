#![cfg_attr(not(feature = "std"), no_std)]

mod array;
mod error;
mod stack_str;
mod stack_vec;
mod utils;
mod uuid;

pub use {array::*, error::*, stack_str::*, stack_vec::*, utils::*, uuid::*};

pub type Result<T> = core::result::Result<T, Error>;

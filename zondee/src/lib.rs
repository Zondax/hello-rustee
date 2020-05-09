#![cfg_attr(not(feature = "std"), no_std)]

mod array;
mod error;
#[cfg(feature = "framework")]
pub mod framework;
mod stack_str;
mod stack_vec;
mod utils;
pub mod wrapper;

pub use {array::*, error::*, stack_str::*, stack_vec::*, utils::*};

pub type Result<T> = core::result::Result<T, Error>;

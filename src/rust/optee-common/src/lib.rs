//! Common types definitions to be used by host and ta
#![no_std]

mod tee_error;
pub use tee_error::{TeeError, TeeErrorCode};

#[repr(u32)]
pub enum CommandId {
    Inc,
    Dec,
    Unknown,
}

impl From<u32> for CommandId {
    fn from(cmd: u32) -> Self {
        match cmd {
            0..=1 => unsafe { core::mem::transmute(cmd) },
            _ => CommandId::Unknown,
        }
    }
}

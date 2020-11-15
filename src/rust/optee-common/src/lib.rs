//! Common types definitions to be used by host and ta
#![no_std]

#[cfg(feature = "ta")]
use zondee_utee::wrapper::Parameters;

mod tee_error;
pub use tee_error::{TeeError, TeeErrorCode};

#[repr(u32)]
pub enum CommandId {
    Encode,
    Decode,
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

#[cfg(feature = "ta")]
/// Trait that must be implemented by types that can process commands from Ta
pub trait HandleTaCommand {
    fn handle_command(
        &mut self,
        cmd_id: u32,
        param_types: u32,
        parameters: &mut Parameters,
    ) -> Result<(), TeeErrorCode>;
}

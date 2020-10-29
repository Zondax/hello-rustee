//! Common types definitions to be used by host and ta
#![no_std]
mod tee_error;
pub use tee_error::{TeeError, TeeErrorCode};

#[repr(u32)]
pub enum CommandId {
    EncryptPhrase,
    InsertUnknown, // not sure if ta is going to process this
    SignWith,
    SignWithAny,
    Sr25519VrfSign,
    Unknown,
}

impl From<u32> for CommandId {
    fn from(cmd: u32) -> Self {
        match cmd {
            0..=4 => unsafe { core::mem::transmute(cmd) },
            _ => CommandId::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// https://raw.githubusercontent.com/mesalock-linux/rust-optee-trustzone-sdk/master/optee-teec/src/uuid.rs

use crate::{
    to_hex,
    wrapper::{client::raw::TEEC_UUID, os},
};
use core::fmt;

pub struct Uuid {
    raw: TEEC_UUID,
}

impl Uuid {
    pub fn from_os_uuid(uuid: os::Uuid) -> Self {
        Self {
            raw: TEEC_UUID {
                timeLow: uuid.timeLow,
                timeMid: uuid.timeMid,
                timeHiAndVersion: uuid.timeHiAndVersion,
                clockSeqAndNode: uuid.clockSeqAndNode,
            },
        }
    }

    /// Converts a uuid to a const raw `TEEC_UUID` pointer.
    pub fn as_ptr(&self) -> *const TEEC_UUID {
        &self.raw
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:x}-{:x}-{:x}-{}",
            self.raw.timeLow,
            self.raw.timeMid,
            self.raw.timeHiAndVersion,
            to_hex(&self.raw.clockSeqAndNode).expect("Bad hex string")
        )
    }
}

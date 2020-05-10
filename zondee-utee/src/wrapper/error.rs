// https://githubcom/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/src/errorrs

#[derive(Debug)]
#[repr(u32)]
pub enum Error {
    /// Object corruption
    CorruptObject = 0xF0100001,
    /// Persistent object corruption
    CorruptObject2 = 0xF0100002,
    /// Object storage is not available
    StorageNotAvailable = 0xF0100003,
    /// Persistent object storage is not available
    StorageNotAvailable2 = 0xF0100004,
    /// Non-specific cause
    Generic = 0xFFFF0000,
    /// Access privileges are not sufficient
    AccessDenied = 0xFFFF0001,
    /// The operation was canceled
    Cancel = 0xFFFF0002,
    /// Concurrent accesses caused conflict
    AccessConflict = 0xFFFF0003,
    /// Too much data for the requested operation was passed
    ExcessData = 0xFFFF0004,
    /// Input data was of invalid format
    BadFormat = 0xFFFF0005,
    /// Input parameters were invalid
    BadParameters = 0xFFFF0006,
    /// Operation is not valid in the current state
    BadState = 0xFFFF0007,
    /// The requested data item is not found
    ItemNotFound = 0xFFFF0008,
    /// The requested operation should exist but is not yet implemented
    NotImplemented = 0xFFFF0009,
    /// The requested operation is valid but is not supported in this implementation
    NotSupported = 0xFFFF000A,
    /// Expected data was missing
    NoData = 0xFFFF000B,
    /// System ran out of resources
    OutOfMemory = 0xFFFF000C,
    /// The system is busy working on something else
    Busy = 0xFFFF000D,
    /// Communication with a remote party failed
    Communication = 0xFFFF000E,
    /// A security fault was detected
    Security = 0xFFFF000F,
    /// The supplied buffer is too short for the generated output
    ShortBuffer = 0xFFFF0010,
    /// The operation has been cancelled by an external event which occurred in the REE while the function was in progress
    ExternalCancel = 0xFFFF0011,
    /// Data overflow
    Overflow = 0xFFFF300F,
    /// Trusted Application has panicked during the operation
    TargetDead = 0xFFFF3024,
    /// Insufficient space is available
    StorageNoSpace = 0xFFFF3041,
    /// MAC is invalid
    MacInvalid = 0xFFFF3071,
    /// Signature is invalid
    SignatureInvalid = 0xFFFF3072,
    /// The persistent time has not been set
    TimeNotSet = 0xFFFF5000,
    /// The persistent time has been set but may have been corrupted and SHALL no longer be trusted
    TimeNeedsReset = 0xFFFF5001,
    /// Unknown error
    Unknown,
}

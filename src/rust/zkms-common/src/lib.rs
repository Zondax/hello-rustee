//! Common definitions
#![deny(
    clippy::expect_used,
    //missing_debug_implementations,
    rust_2018_idioms,
    trivial_casts,
    //unsafe_code,
    unused_lifetimes
)]

pub mod jsonrpc_request;
pub mod transferable_transcript;
pub mod utils;

pub use jsonrpc_request::{HandlerRequest, KeystoreRequest, KeystoreResponse, RequestMethod};
pub use transferable_transcript::TransferableVRFTranscriptData;
pub use utils::{deserialize, serialize};

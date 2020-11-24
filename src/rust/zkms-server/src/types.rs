//! Types module definition
//!
use jsonrpc_core::BoxFuture;
use jsonrpc_derive::rpc;

#[rpc]
pub trait RemoteSignerApi {
    /// Returns all sr25519 public keys for the given key type.
    #[rpc(name = "increment", returns = "u64")]
    fn increment(&self, msg: u64) -> BoxFuture<u64>;

    /// Returns all sr25519 public keys for the given key type.
    #[rpc(name = "decrement", returns = "u64")]
    fn decrement(&self, msg: u64) -> BoxFuture<u64>;
}

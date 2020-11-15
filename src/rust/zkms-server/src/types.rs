//! Types module definition
//!
use jsonrpc_core::BoxFuture;
use jsonrpc_derive::rpc;

#[rpc]
pub trait RemoteSignerApi {
    /// Returns all sr25519 public keys for the given key type.
    #[rpc(name = "encode", returns = "String")]
    fn encode(&self, msg: String) -> BoxFuture<String>;

    /// Returns all sr25519 public keys for the given key type.
    #[rpc(name = "decode", returns = "String")]
    fn decode(&self, msg: String) -> BoxFuture<String>;
}

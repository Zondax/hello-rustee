//! Server module
use sp_core::{
    crypto::{CryptoTypePublicPair, KeyTypeId},
    ecdsa, ed25519, sr25519,
};
use sp_keystore::vrf::VRFSignature;

use jsonrpc_core::{BoxFuture, Error as RpcError};

use futures::{
    channel::oneshot,
    future::{FutureExt, TryFutureExt},
};

use crate::types::RemoteSignerApi;
use zkms_common::{
    HandlerRequest, KeystoreRequest, KeystoreResponse, RequestMethod, TransferableVRFTranscriptData,
};

/// Handler for requests
pub struct ServerHandler<Handler> {
    handler: Handler,
}

impl<Handler: HandlerRequest + Default + 'static> Default for ServerHandler<Handler> {
    fn default() -> ServerHandler<Handler> {
        let handler = Handler::default();
        Self { handler }
    }
}

impl<Handler: HandlerRequest + 'static> ServerHandler<Handler> {
    /// Creates a new server handler instance
    /// # Arguments
    ///
    /// * `store` - Any type that implements the HandlerRequest trait
    pub fn new(store: Handler) -> ServerHandler<Handler> {
        Self { handler: store }
    }

    // TODO: A real implementation might require this function to be async
    // It would depend on the inner TEE interface and other interfaces at hand at that moment
    fn process_request(&self, request: RequestMethod) -> oneshot::Receiver<KeystoreResponse> {
        let (request_sender, receiver) = oneshot::channel::<KeystoreResponse>();

        let request = KeystoreRequest {
            sender: request_sender,
            method: request,
        };

        self.handler
            .process_request(request)
            .expect("This Should not fail");

        receiver
    }
}

impl<Handler: HandlerRequest + 'static> RemoteSignerApi for ServerHandler<Handler> {
    fn sr25519_public_keys(&self, id: KeyTypeId) -> BoxFuture<Vec<sr25519::Public>> {
        let receiver = self.process_request(RequestMethod::Sr25519PublicKeys(id));
        Box::new(
            receiver
                .map(|e| match e {
                    Ok(KeystoreResponse::Sr25519PublicKeys(keys)) => Ok(keys),
                    _ => Ok(vec![]),
                })
                .boxed()
                .compat(),
        )
    }

    fn sr25519_generate_new(
        &self,
        id: KeyTypeId,
        seed: Option<String>,
    ) -> BoxFuture<sr25519::Public> {
        Box::new(
            self.process_request(RequestMethod::Sr25519GenerateNew(id, seed))
                .map(|response| {
                    if let Ok(KeystoreResponse::Sr25519GenerateNew(result)) = response {
                        result.map_err(|_| RpcError::internal_error())
                    } else {
                        Err(RpcError::internal_error())
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn ed25519_public_keys(&self, id: KeyTypeId) -> BoxFuture<Vec<ed25519::Public>> {
        Box::new(
            self.process_request(RequestMethod::Ed25519PublicKeys(id))
                .map(|response| {
                    if let Ok(KeystoreResponse::Ed25519PublicKeys(keys)) = response {
                        Ok(keys)
                    } else {
                        Ok(vec![])
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn ed25519_generate_new(
        &self,
        id: KeyTypeId,
        seed: Option<String>,
    ) -> BoxFuture<ed25519::Public> {
        Box::new(
            self.process_request(RequestMethod::Ed25519GenerateNew(id, seed))
                .map(|response| {
                    if let Ok(KeystoreResponse::Ed25519GenerateNew(result)) = response {
                        result.map_err(|_| RpcError::internal_error())
                    } else {
                        Err(RpcError::internal_error())
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn ecdsa_public_keys(&self, id: KeyTypeId) -> BoxFuture<Vec<ecdsa::Public>> {
        Box::new(
            self.process_request(RequestMethod::EcdsaPublicKeys(id))
                .map(|response| {
                    if let Ok(KeystoreResponse::EcdsaPublicKeys(keys)) = response {
                        Ok(keys)
                    } else {
                        Ok(vec![])
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn ecdsa_generate_new(&self, id: KeyTypeId, seed: Option<String>) -> BoxFuture<ecdsa::Public> {
        Box::new(
            self.process_request(RequestMethod::EcdsaGenerateNew(id, seed))
                .map(|response| {
                    if let Ok(KeystoreResponse::EcdsaGenerateNew(result)) = response {
                        result.map_err(|_| RpcError::internal_error())
                    } else {
                        Err(RpcError::internal_error())
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn insert_unknown(&self, key_type: KeyTypeId, suri: String, public: Vec<u8>) -> BoxFuture<()> {
        Box::new(
            self.process_request(RequestMethod::InsertUnknown(key_type, suri, public))
                .map(|_| Ok(()))
                .boxed()
                .compat(),
        )
    }

    fn supported_keys(
        &self,
        id: KeyTypeId,
        keys: Vec<CryptoTypePublicPair>,
    ) -> BoxFuture<Vec<CryptoTypePublicPair>> {
        Box::new(
            self.process_request(RequestMethod::SupportedKeys(id, keys))
                .map(|response| {
                    if let Ok(KeystoreResponse::SupportedKeys(keys)) = response {
                        keys.map_err(|e| RpcError::invalid_params(e.to_string()))
                    } else {
                        Ok(vec![])
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn keys(&self, id: KeyTypeId) -> BoxFuture<Vec<CryptoTypePublicPair>> {
        Box::new(
            self.process_request(RequestMethod::Keys(id))
                .map(|response| {
                    if let Ok(KeystoreResponse::Keys(keys)) = response {
                        keys.map_err(|e| RpcError::invalid_params(e.to_string()))
                    } else {
                        Ok(vec![])
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn has_keys(&self, public_keys: Vec<(Vec<u8>, KeyTypeId)>) -> BoxFuture<bool> {
        Box::new(
            self.process_request(RequestMethod::HasKeys(public_keys.to_vec()))
                .map(|response| {
                    if let Ok(KeystoreResponse::HasKeys(exists)) = response {
                        Ok(exists)
                    } else {
                        Ok(false)
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn sign_with(
        &self,
        id: KeyTypeId,
        key: CryptoTypePublicPair,
        msg: Vec<u8>,
    ) -> BoxFuture<Vec<u8>> {
        Box::new(
            self.process_request(RequestMethod::SignWith(id, key, msg))
                .map(|response| {
                    if let Ok(KeystoreResponse::SignWith(result)) = response {
                        result.map_err(|e| RpcError::invalid_params(e.to_string()))
                    } else {
                        Err(RpcError::internal_error())
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn sign_with_any(
        &self,
        id: KeyTypeId,
        keys: Vec<CryptoTypePublicPair>,
        msg: Vec<u8>,
    ) -> BoxFuture<(CryptoTypePublicPair, Vec<u8>)> {
        Box::new(
            self.process_request(RequestMethod::SignWithAny(id, keys, msg))
                .map(|response| {
                    if let Ok(KeystoreResponse::SignWithAny(result)) = response {
                        result.map_err(|e| RpcError::invalid_params(e.to_string()))
                    } else {
                        Err(RpcError::internal_error())
                    }
                })
                .boxed()
                .compat(),
        )
    }

    fn sign_with_all(
        &self,
        _id: KeyTypeId,
        _keys: Vec<CryptoTypePublicPair>,
        _msg: Vec<u8>,
    ) -> BoxFuture<Vec<Result<Vec<u8>, String>>> {
        todo!();
    }

    fn sr25519_vrf_sign(
        &self,
        key_type: KeyTypeId,
        public: sr25519::Public,
        transcript_data: TransferableVRFTranscriptData,
    ) -> BoxFuture<VRFSignature> {
        Box::new(
            self.process_request(RequestMethod::Sr25519VrfSign(
                key_type,
                public,
                transcript_data,
            ))
            .map(|response| {
                if let Ok(KeystoreResponse::Sr25519VrfSign(result)) = response {
                    result.map_err(|e| RpcError::invalid_params(e.to_string()))
                } else {
                    Err(RpcError::internal_error())
                }
            })
            .boxed()
            .compat(),
        )
    }
}

#[cfg(test)]
mod tests {
    use jsonrpc_test;
    use serde_json;
    use tokio;

    use super::*;
    use crate::{local_handler::LocalHandler, types::RemoteSignerApi};

    const TEST_TK: KeyTypeId = KeyTypeId(*b"test");
    const TEST_TK_NOPE: KeyTypeId = KeyTypeId(*b"nope");

    async fn setup() -> jsonrpc_test::Rpc {
        let server = ServerHandler::<LocalHandler>::default();
        jsonrpc_test::Rpc::new(RemoteSignerApi::to_delegate(server))
    }

    #[tokio::test(core_threads = 4)]
    async fn test_keys() {
        let rpc = setup().await;

        let _ = rpc.request("signer_sr25519_generate_new", &(TEST_TK, Some("//Alice")));
        let _ = rpc.request("signer_ed25519_generate_new", &(TEST_TK, Some("//Bob")));
        let _ = rpc.request("signer_ecdsa_generate_new", &(TEST_TK, Some("//Charlie")));

        let r = rpc.request("signer_keys", &[TEST_TK]);

        let res: Vec<CryptoTypePublicPair> = serde_json::from_str(&r).unwrap();

        assert_eq!(res.len(), 9);

        let r = rpc.request("signer_keys", &[TEST_TK_NOPE]);
        let res: Vec<CryptoTypePublicPair> = serde_json::from_str(&r).unwrap();
        assert_eq!(res.len(), 0);
    }

    #[tokio::test(core_threads = 4)]
    async fn test_sr25519_public_keys() {
        let rpc = setup().await;
        let _ = rpc.request("signer_sr25519_generate_new", &(TEST_TK, Some("//Alice")));

        let r = rpc.request("signer_sr25519_public_keys", &[TEST_TK]);
        let res: Vec<sr25519::Public> = serde_json::from_str(&r).unwrap();
        assert_eq!(res.len(), 1);

        let r = rpc.request("signer_sr25519_public_keys", &[TEST_TK_NOPE]);
        let res: Vec<sr25519::Public> = serde_json::from_str(&r).unwrap();
        assert_eq!(res.len(), 0);
    }

    #[tokio::test(core_threads = 4)]
    async fn test_ed25519_public_keys() {
        let rpc = setup().await;
        let _ = rpc.request("signer_ed25519_generate_new", &(TEST_TK, Some("//Bob")));
        let _ = rpc.request("signer_ed25519_generate_new", &(TEST_TK, Some("//Bob2")));
        let r = rpc.request("signer_ed25519_public_keys", &[TEST_TK]);
        let res: Vec<ed25519::Public> = serde_json::from_str(&r).unwrap();
        assert_eq!(res.len(), 2);

        let r = rpc.request("signer_ed25519_public_keys", &[TEST_TK_NOPE]);
        let res: Vec<ed25519::Public> = serde_json::from_str(&r).unwrap();
        assert_eq!(res.len(), 0);
    }
}

//! Request handler module that uses the LocakKeystore which is a sync implementation of the CryptoStore trait from parity/substrate

use sp_keystore::SyncCryptoStore;

use wasmtime::Val;

use zkms_common::{HandlerRequest, KeystoreRequest, KeystoreResponse, RequestMethod};

use crate::{Executor, WASM_BINARY};
use sc_keystore::LocalKeystore;

/// Local handler(Sync)
pub struct LocalHandler {
    store: LocalKeystore,
    executor: Executor,
}

impl Default for LocalHandler {
    fn default() -> Self {
        let store = LocalKeystore::in_memory();
        let executor = Executor::new(WASM_BINARY).expect("Invalid wasm code");
        Self { store, executor }
    }
}

impl LocalHandler {
    /// Creates in memory a local key store manager
    pub fn new() -> Self {
        Self::default()
    }
}

impl HandlerRequest for LocalHandler {
    fn process_request(&self, request: KeystoreRequest) -> Result<(), String> {
        let sender = request.sender;
        match request.method {
            RequestMethod::SignWith(id, key, msg) => {
                // Check if the id value is supported by our wasm_runtime
                let wasm_args = [Val::I32(u32::from(id) as i32)];
                match self.executor.call("check_key_id", Some(&wasm_args)) {
                    Ok(valid) if (valid.len() > 0) && (valid[0].i32() > Some(0)) => {}
                    Ok(_) => return Err("Invalid key id type".to_string()),
                    Err(e) => return Err(e),
                }
                let result = self.store.sign_with(id, &key, &msg);
                sender
                    .send(KeystoreResponse::SignWith(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::SignWithAny(id, key, msg) => {
                let result = self.store.sign_with_any(id, key, &msg);
                sender
                    .send(KeystoreResponse::SignWithAny(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::Sr25519PublicKeys(id) => {
                let result = self.store.sr25519_public_keys(id);
                sender
                    .send(KeystoreResponse::Sr25519PublicKeys(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::Sr25519VrfSign(id, public, data) => {
                let transcript_data = data.into();
                let result = self.store.sr25519_vrf_sign(id, &public, transcript_data);
                sender
                    .send(KeystoreResponse::Sr25519VrfSign(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::Sr25519GenerateNew(id, seed) => {
                let result = self.store.sr25519_generate_new(id, seed.as_deref());
                sender
                    .send(KeystoreResponse::Sr25519GenerateNew(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::Ed25519PublicKeys(id) => {
                let result = self.store.ed25519_public_keys(id);
                sender
                    .send(KeystoreResponse::Ed25519PublicKeys(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::Ed25519GenerateNew(id, seed) => {
                let result = self.store.ed25519_generate_new(id, seed.as_deref());
                sender
                    .send(KeystoreResponse::Ed25519GenerateNew(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::EcdsaPublicKeys(id) => {
                let result = self.store.ecdsa_public_keys(id);
                sender
                    .send(KeystoreResponse::EcdsaPublicKeys(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::EcdsaGenerateNew(id, seed) => {
                let result = self.store.ecdsa_generate_new(id, seed.as_deref());
                sender
                    .send(KeystoreResponse::EcdsaGenerateNew(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::HasKeys(keys) => {
                let result = self.store.has_keys(&keys);
                sender
                    .send(KeystoreResponse::HasKeys(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::SupportedKeys(id, keys) => {
                let result = self.store.supported_keys(id, keys);
                sender
                    .send(KeystoreResponse::SupportedKeys(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::Keys(id) => {
                let result = self.store.keys(id);
                sender
                    .send(KeystoreResponse::Keys(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
            RequestMethod::InsertUnknown(key_type, suri, pubkey) => {
                let result = self.store.insert_unknown(key_type, suri.as_str(), &pubkey);
                sender
                    .send(KeystoreResponse::InsertUnknown(result))
                    .map_err(|_| "The receiver is disconnected".to_string())?;
            }
        }
        Ok(())
    }
}

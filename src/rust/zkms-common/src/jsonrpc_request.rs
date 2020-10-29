// https://github.com/paritytech/substrate/blob/ben-remote-signer/client/jsonrpc-remote-signer/src/server.rs

use futures::channel::oneshot;
use sp_core::{
    crypto::{CryptoTypePublicPair, KeyTypeId},
    ecdsa, ed25519, sr25519,
};

use sp_keystore::{vrf::VRFSignature, Error};

use crate::transferable_transcript::*;

/// Request handler interface
pub trait HandlerRequest: Send + Sync {
    /// process a request
    fn process_request(&self, request: KeystoreRequest) -> Result<(), String>;
}

// First para is the keyTypeId
//let p0 = ParamValue::new(key_type, 0, ParamType::ValueInput);
// then the input seed if present
#[derive(serde::Deserialize, serde::Serialize)]
pub enum RequestMethod {
    Sr25519PublicKeys(KeyTypeId),
    Sr25519GenerateNew(KeyTypeId, Option<String>),
    Sr25519VrfSign(KeyTypeId, sr25519::Public, TransferableVRFTranscriptData),
    Ed25519PublicKeys(KeyTypeId),
    Ed25519GenerateNew(KeyTypeId, Option<String>),
    EcdsaPublicKeys(KeyTypeId),
    EcdsaGenerateNew(KeyTypeId, Option<String>),
    InsertUnknown(KeyTypeId, String, Vec<u8>),
    SupportedKeys(KeyTypeId, Vec<CryptoTypePublicPair>),
    Keys(KeyTypeId),
    HasKeys(Vec<(Vec<u8>, KeyTypeId)>),
    SignWith(KeyTypeId, CryptoTypePublicPair, Vec<u8>),
    SignWithAny(KeyTypeId, Vec<CryptoTypePublicPair>, Vec<u8>),
}

pub struct KeystoreRequest {
    pub sender: oneshot::Sender<KeystoreResponse>,
    pub method: RequestMethod,
}

pub enum KeystoreResponse {
    Sr25519PublicKeys(Vec<sr25519::Public>),
    Sr25519GenerateNew(Result<sr25519::Public, Error>),
    Sr25519VrfSign(Result<VRFSignature, Error>),
    Ed25519PublicKeys(Vec<ed25519::Public>),
    Ed25519GenerateNew(Result<ed25519::Public, Error>),
    EcdsaPublicKeys(Vec<ecdsa::Public>),
    EcdsaGenerateNew(Result<ecdsa::Public, Error>),
    InsertUnknown(Result<(), ()>),
    SupportedKeys(Result<Vec<CryptoTypePublicPair>, Error>),
    Keys(Result<Vec<CryptoTypePublicPair>, Error>),
    HasKeys(bool),
    SignWith(Result<Vec<u8>, Error>),
    SignWithAny(Result<(CryptoTypePublicPair, Vec<u8>), Error>),
}

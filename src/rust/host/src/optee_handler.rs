use parking_lot::RwLock;
use std::fs::File;
use std::io::Write;

use std::{collections::HashSet, path::PathBuf};
use zkms_common::{serialize, HandlerRequest, KeystoreRequest, KeystoreResponse, RequestMethod};

use optee_common::CommandId;

use crate::invoke_command;

use optee_teec::{Operation, ParamNone, ParamTmpRef, ParamType, ParamValue};

use sp_application_crypto::{ecdsa, ed25519, sr25519};
use sp_core::{
    crypto::{CryptoTypePublicPair, ExposeSecret, KeyTypeId, Pair as PairT, Public, SecretString},
    sr25519::{Pair as Sr25519Pair, Public as Sr25519Public},
    Encode,
};
use sp_keystore::Error;

use zkms_keystore::KeyStore;

pub struct Handler(RwLock<KeyStore>);

impl Handler {
    /// Create a local keystore from filesystem.                                                     
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<Self, String> {
        let inner = KeyStore::open(path).map_err(|e| e.to_string())?;
        Ok(Self(RwLock::new(inner)))
    }

    /// Create a local keystore in memory.                                                           
    pub fn in_memory() -> Self {
        let inner = KeyStore::new();
        Self(RwLock::new(inner))
    }

    fn keys(&self, id: KeyTypeId) -> std::result::Result<Vec<CryptoTypePublicPair>, Error> {
        let raw_keys = self.0.read().raw_public_keys(id)?;
        Ok(raw_keys.into_iter().fold(Vec::new(), |mut v, k| {
            v.push(CryptoTypePublicPair(sr25519::CRYPTO_ID, k.clone()));
            v.push(CryptoTypePublicPair(ed25519::CRYPTO_ID, k.clone()));
            v.push(CryptoTypePublicPair(ecdsa::CRYPTO_ID, k));
            v
        }))
    }

    fn generate_pair<Pair: PairT, S: AsRef<str>>(
        &self,
        key_type: KeyTypeId,
        seed: Option<S>,
    ) -> Result<Pair, Error> {
        let (pair, phrase) = match seed {
            Some(ref seed) => {
                let pair = Pair::from_string(seed.as_ref(), None)
                    .map_err(|_| Error::Other("Invalid seed".into()))?;
                (pair, seed.as_ref().into())
            }
            None => {
                let (pair, phrase, _) = Pair::generate_with_phrase(None);
                (pair, phrase)
            }
        };
        // The AES256 encryption result is 32-bytes, for now we just reserver enought space for getting back the same
        // phrase
        let mut encrypted = vec![0u8; phrase.len()];
        // The encryption result, not needed just in case
        //let mut encrypted_len = 0u32;

        let mut phrase_bytes = phrase.into_bytes();
        let p0 = ParamTmpRef::new_input(phrase_bytes.as_mut());
        let p1 = ParamTmpRef::new_output(encrypted.as_mut());
        let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);
        // Calls the TA to encrypt this phrase
        invoke_command(CommandId::EncryptPhrase as _, &mut operation)
            .map_err(|e| Error::Other(e.to_string()))?;

        let encrypted_phrase = hex::encode(encrypted.as_slice());

        // Check if the key/phrase has to be stored in memory or file system
        if seed.is_some() {
            // Stores the key in memory
            self.0.write().insert_ephemeral_pair(
                &pair,
                encrypted_phrase.as_str(),
                key_type.clone(),
            );
        } else {
            // Stores the key in the file system
            if let Some(path) = self
                .0
                .read()
                .key_file_path(pair.public().as_slice(), key_type)
            {
                let mut file = File::create(path).map_err(|e| Error::Other(e.to_string()))?;
                serde_json::to_writer(&file, &encrypted_phrase)
                    .map_err(|e| Error::Other(e.to_string()))?;
                file.flush().map_err(|e| Error::Other(e.to_string()))?;
            }
        }

        Ok(pair)
    }
}

/*
pub enum RequestMethod {
    Sr25519PublicKeys(KeyTypeId),
    Sr25519GenerateNew(KeyTypeId, Option<String>),
    Sr25519VrfSign(KeyTypeId, sr25519::Public, VRFTranscriptData),
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

pub enum KeystoreResponse {
    Sr25519PublicKeys(Vec<sr25519::Public>),
    Sr25519GenerateNew(Result<sr25519::Public, CryptoStoreError>),
    Sr25519VrfSign(Result<VRFSignature, CryptoStoreError>),
    Ed25519PublicKeys(Vec<ed25519::Public>),
    Ed25519GenerateNew(Result<ed25519::Public, CryptoStoreError>),
    EcdsaPublicKeys(Vec<ecdsa::Public>),
    EcdsaGenerateNew(Result<ecdsa::Public, CryptoStoreError>),
    InsertUnknown(Result<(), ()>),
    SupportedKeys(Result<Vec<CryptoTypePublicPair>, CryptoStoreError>),
    Keys(Result<Vec<CryptoTypePublicPair>, CryptoStoreError>),
    HasKeys(bool),
    SignWith(Result<Vec<u8>, CryptoStoreError>),
    SignWithAny(Result<(CryptoTypePublicPair, Vec<u8>), CryptoStoreError>),
}
*/

impl HandlerRequest for Handler {
    fn process_request(&self, request: KeystoreRequest) -> Result<(), String> {
        let sender = request.sender;
        match request.method {
            RequestMethod::SignWith(id, key, msg) => {

                // Here we get the phrase from our in_memory/file system storage
                // them call TA to decrypt it, once it is decrypted, proceed to create
                // a Pair and pass this Pair + msg to the TA, which would check the validity of the msg and sign it if
                // it is so.

                /*let mut serial_data =
                    bincode::serialize(&(id, key, msg)).map_err(|e| e.to_string())?;
                let p0 = ParamTmpRef::new_input(serial_data.as_mut());
                // If the key exists them call the TA, passing the encrypted seed and message,
                // the TA can get the priv key from the encrypted seed and sign the message
                let mut sign_out = vec![0u8; 128];
                let p1 = ParamTmpRef::new_output(sign_out.as_mut());
                let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

                invoke_command(CommandId::SignWith as _, &mut operation)
                    .map_err(|e| e.to_string())?;
                sender.send(KeystoreResponse::SignWith(Ok(sign_out)));*/
            }
            RequestMethod::SignWithAny(id, key, msg) => {
                /*let mut serial_data =
                    bincode::serialize(&(id, key, msg)).map_err(|e| e.to_string())?;
                let p0 = ParamTmpRef::new_input(serial_data.as_mut());
                // If the key exists them call the TA, passing the encrypted seed and message,
                // the TA can get the priv key from the encrypted seed and sign the message
                let mut sign_out = (CryptoTypePublicPair::default(), vec![0u8; 128]);
                let mut sign_out_vec = bincode::serialize(&sign_out).map_err(|e| e.to_string())?;
                let p1 = ParamTmpRef::new_output(sign_out_vec.as_mut());
                let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

                invoke_command(CommandId::SignWithAny as _, &mut operation)
                    .map_err(|e| e.to_string())?;
                // TODO check this
                sender.send(KeystoreResponse::SignWithAny(Ok(sign_out)));*/
            }
            RequestMethod::Sr25519VrfSign(id, public, data) => {}
            RequestMethod::Sr25519GenerateNew(id, seed) => {
                let pub_key = self
                    .generate_pair::<sr25519::Pair, _>(id, seed)
                    .map_err(|e| e.to_string())?
                    .public();
                let _ = sender.send(KeystoreResponse::Sr25519GenerateNew(Ok(pub_key)));
            }
            RequestMethod::Ed25519GenerateNew(id, seed) => {
                let pub_key = self
                    .generate_pair::<ed25519::Pair, _>(id, seed)
                    .map_err(|e| e.to_string())?
                    .public();
                let _ = sender.send(KeystoreResponse::Ed25519GenerateNew(Ok(pub_key)));
            }
            RequestMethod::EcdsaGenerateNew(id, seed) => {
                let pub_key = self
                    .generate_pair::<ecdsa::Pair, _>(id, seed)
                    .map_err(|e| e.to_string())?
                    .public();
                let _ = sender.send(KeystoreResponse::EcdsaGenerateNew(Ok(pub_key)));
            }
            RequestMethod::Sr25519PublicKeys(key_type) => {
                let result = self
                    .0
                    .read()
                    .raw_public_keys(key_type)
                    .map(|v| {
                        v.into_iter()
                            .map(|k| sr25519::Public::from_slice(k.as_slice()))
                            .collect()
                    })
                    .unwrap_or_default();

                let _ = sender.send(KeystoreResponse::Sr25519PublicKeys(result));
            }
            RequestMethod::Ed25519PublicKeys(key_type) => {
                let result = self
                    .0
                    .read()
                    .raw_public_keys(key_type)
                    .map(|v| {
                        v.into_iter()
                            .map(|k| ed25519::Public::from_slice(k.as_slice()))
                            .collect()
                    })
                    .unwrap_or_default();

                let _ = sender.send(KeystoreResponse::Ed25519PublicKeys(result));
            }
            RequestMethod::EcdsaPublicKeys(key_type) => {
                let result = self
                    .0
                    .read()
                    .raw_public_keys(key_type)
                    .map(|v| {
                        v.into_iter()
                            .map(|k| ecdsa::Public::from_slice(k.as_slice()))
                            .collect()
                    })
                    .unwrap_or_default();

                let _ = sender.send(KeystoreResponse::EcdsaPublicKeys(result));
            }
            RequestMethod::HasKeys(keys) => {
                let _ = sender
                    .send(KeystoreResponse::HasKeys(keys.iter().all(|(p, t)| {
                        self.0.read().key_phrase_by_type(&p, *t).is_ok()
                    })));
            }
            RequestMethod::SupportedKeys(id, keys) => {
                let all_keys = self
                    .keys(id)
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .collect::<HashSet<_>>();

                let _ = sender.send(KeystoreResponse::SupportedKeys(Ok(keys
                    .into_iter()
                    .filter(|key| all_keys.contains(key))
                    .collect::<Vec<_>>())));
            }
            RequestMethod::Keys(id) => {
                let _ = sender.send(KeystoreResponse::Keys(self.keys(id)));
            }

            RequestMethod::InsertUnknown(key_type, ref suri, ref pubkey) => {
                let _ = sender.send(KeystoreResponse::InsertUnknown(
                    self.0
                        .write()
                        .insert_unknown(key_type, suri, pubkey)
                        .map_err(|_| ()),
                ));
            }
        }
        Ok(())
    }
}

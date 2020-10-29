// https://github.com/paritytech/substrate/blob/master/client/keystore/src/local.rs
use sc_keystore::{Error, Result};
use sp_application_crypto::{ecdsa, ed25519, sr25519};
use sp_core::{
    crypto::{CryptoTypePublicPair, ExposeSecret, KeyTypeId, Pair as PairT, Public, SecretString},
    sr25519::{Pair as Sr25519Pair, Public as Sr25519Public},
    Encode,
};

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

/// A KeyStore object to handle key operations and storing keys in memory or in a more persistent
/// resource a files
pub struct KeyStore {
    path: Option<PathBuf>,
    // Map over `(KeyTypeId, Raw public key)` -> `Key phrase/seed`
    // the phrase here is encrypted
    in_memory: HashMap<(KeyTypeId, Vec<u8>), String>,
}

impl Default for KeyStore {
    fn default() -> Self {
        Self {
            path: None,
            in_memory: HashMap::new(),
        }
    }
}

impl KeyStore {
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<Self> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let instance = Self {
            path: Some(path),
            in_memory: HashMap::new(),
        };
        Ok(instance)
    }
    /// Creates in memory a local key store manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the file path for the given public key and key type.                                 
    pub fn key_file_path(&self, public: &[u8], key_type: KeyTypeId) -> Option<PathBuf> {
        let mut buf = self.path.as_ref()?.clone();
        let key_type = hex::encode(key_type.0);
        let key = hex::encode(public);
        buf.push(key_type + key.as_str());
        Some(buf)
    }

    /// Create a new key from seed.                                                                  
    ///                                                                                              
    /// At this pointe the seed has been already encrypted. Does not place it into the file system store.                                                
    pub fn insert_ephemeral_from_seed_by_type<Pair: PairT>(
        &mut self,
        seed: &str,
        key_type: KeyTypeId,
    ) -> Result<Pair> {
        let pair = Pair::from_string(seed, None).map_err(|_| Error::InvalidSeed)?;
        self.insert_ephemeral_pair(&pair, seed, key_type);
        Ok(pair)
    }

    /// Insert the given public/private key pair with the given key type.                            
    ///                                                                                              
    /// Does not place it into the file system store.                                                
    pub fn insert_ephemeral_pair<Pair: PairT>(
        &mut self,
        pair: &Pair,
        seed: &str,
        key_type: KeyTypeId,
    ) {
        let key = (key_type, pair.public().to_raw_vec());
        self.in_memory.insert(key, seed.into());
    }

    /// Insert a new key with anonymous crypto.                                                      
    ///                                                                                              
    /// Places it into the file system store.                                                        
    pub fn insert_unknown(&self, key_type: KeyTypeId, suri: &str, public: &[u8]) -> Result<()> {
        if let Some(path) = self.key_file_path(public, key_type) {
            let mut file = File::create(path).map_err(Error::Io)?;
            serde_json::to_writer(&file, &suri).map_err(Error::Json)?;
            file.flush().map_err(Error::Io)?;
        }
        Ok(())
    }

    /// Returns a list of raw public keys filtered by `KeyTypeId`                                    
    pub fn raw_public_keys(&self, id: KeyTypeId) -> Result<Vec<Vec<u8>>> {
        let mut public_keys: Vec<Vec<u8>> = self
            .in_memory
            .keys()
            .into_iter()
            .filter_map(|k| if k.0 == id { Some(k.1.clone()) } else { None })
            .collect();

        if let Some(path) = &self.path {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let path = entry.path();

                // skip directories and non-unicode file names (hex is unicode)
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    match hex::decode(name) {
                        Ok(ref hex) if hex.len() > 4 => {
                            if &hex[0..4] != &id.0 {
                                continue;
                            }
                            let public = hex[4..].to_vec();
                            public_keys.push(public);
                        }
                        _ => continue,
                    }
                }
            }
        }

        Ok(public_keys)
    }

    /// Get the key phrase for a given public key and key type.                                                                                                        
    pub fn key_phrase_by_type(&self, public: &[u8], key_type: KeyTypeId) -> Result<String> {
        if let Some(phrase) = self.get_additional_pair(public, key_type) {
            return Ok(phrase.clone());
        }

        let path = self
            .key_file_path(public, key_type)
            .ok_or_else(|| Error::Unavailable)?;
        let file = File::open(path)?;

        serde_json::from_reader(&file).map_err(Into::into)
    }

    /// Get the key phrase for the given public key and key type from the in-memory store.                                                                             
    fn get_additional_pair(&self, public: &[u8], key_type: KeyTypeId) -> Option<&String> {
        let key = (key_type, public.to_vec());
        self.in_memory.get(&key)
    }

    /// Get a key pair for the given public key and key type.                                        
    pub fn key_pair_by_type<Pair: PairT>(
        &self,
        public: &Pair::Public,
        key_type: KeyTypeId,
    ) -> Result<Pair> {
        let phrase = self.key_phrase_by_type(public.as_slice(), key_type)?;
        let pair = Pair::from_string(&phrase, None).map_err(|_| Error::InvalidPhrase)?;

        if &pair.public() == public {
            Ok(pair)
        } else {
            Err(Error::InvalidPassword)
        }
    }
}

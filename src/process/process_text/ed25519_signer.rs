use super::{KeyGenerator, KeyLoader, TextSign};
use anyhow::Result;
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

pub struct Ed25519Signer {
    pub key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let from_bytes_key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(from_bytes_key))
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec)?;
        let sign = self.key.sign(&vec);
        Ok(sign.to_vec())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut os_rng = OsRng;
        let sk = SigningKey::generate(&mut os_rng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

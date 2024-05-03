use super::{KeyLoader, TextVerify};
use anyhow::Result;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::{fs, io::Read, path::Path};

pub struct Ed25519Verifier {
    pub key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let vec = fs::read(path)?;
        Self::try_new(&vec)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec)?;
        let sig = Signature::from_bytes(signature.try_into()?);
        Ok(self.key.verify(&vec, &sig).is_ok())
    }
}

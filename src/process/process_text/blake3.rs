use super::{KeyGenerator, KeyLoader, TextSign, TextVerify};
use crate::gen_pass;
use anyhow::Result;
use std::{fs, io::Read, path::Path};

pub struct Blake3 {
    pub key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Self::new(key))
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec)?;
        let keyed_hash = blake3::keyed_hash(&self.key, &vec);
        Ok(keyed_hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec)?;
        let hash = blake3::keyed_hash(&self.key, &vec);
        Ok(hash.as_bytes() == signature)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = gen_pass(32, false, false, false, false)?;
        let key = key.into_bytes();
        Ok(vec![key])
    }
}

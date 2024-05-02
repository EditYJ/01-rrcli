use crate::{gen_pass, utils::get_reader_from_path, TextSignFormatType};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

pub fn sign_text(text: &str, key: &str, format: TextSignFormatType) -> Result<String> {
    let mut reader = get_reader_from_path(text)?;
    let signed = match format {
        TextSignFormatType::Blake3 => {
            let blake3 = Blake3::load(key)?;
            blake3.sign(&mut reader)?
        }
        TextSignFormatType::Ed25519 => {
            let ed25519 = Ed25519::load(key)?;
            ed25519.sign(&mut reader)?
        }
    };

    let signed = URL_SAFE_NO_PAD.encode(signed);

    Ok(signed)
}

pub fn verify_text(input: &str, key: &str, sig: &str, format: TextSignFormatType) -> Result<bool> {
    let mut reader = get_reader_from_path(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;

    match format {
        TextSignFormatType::Blake3 => {
            let blake3 = Blake3::load(key)?;
            blake3.verify(&mut reader, &sig)
        }
        TextSignFormatType::Ed25519 => {
            let ed25519 = Ed25519::load(key)?;
            ed25519.verify(&mut reader, &sig)
        }
    }
}

pub fn generate_key(format: TextSignFormatType) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormatType::Blake3 => Blake3::generate(),
        TextSignFormatType::Ed25519 => Ed25519::generate(),
    }
}

// 文件/文字签名
pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

// 文件/文字验签
pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool>;
}

// 密钥加载
pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

// 密钥生成
pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

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

pub struct Ed25519 {
    pub key: SigningKey,
}

impl Ed25519 {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let from_bytes_key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(from_bytes_key))
    }
}

impl TextSign for Ed25519 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec)?;
        let sign = self.key.sign(&vec);
        Ok(sign.to_vec())
    }
}

impl TextVerify for Ed25519 {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec)?;
        let from_bytes = Signature::from_bytes(signature.try_into()?);
        let verify = self.key.verify(&vec, &from_bytes);
        Ok(verify.is_ok())
    }
}

impl KeyLoader for Ed25519 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut os_rng = OsRng;
        let sk = SigningKey::generate(&mut os_rng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blake3_sign_text() -> Result<()> {
        let brick3 = Blake3::load("fixtures/process_text/key.txt")?;
        let data = b"hello world!";
        let sign = brick3.sign(&mut &data[..])?;
        let sig = brick3.verify(&mut &data[..], &sign)?;
        assert!(sig);

        Ok(())
    }

    #[test]
    fn test_ed25519_sign_text() -> Result<()> {
        let sk = Ed25519::load("fixtures/process_text/ed25519.sk")?;
        let data = b"hello world!";
        let sign = sk.sign(&mut &data[..])?;
        let sig = sk.verify(&mut &data[..], &sign)?;
        assert!(sig);

        Ok(())
    }
}

mod blake3;
mod ed25519_signer;
mod ed25519_verifier;

use crate::{utils::get_reader_from_path, TextSignFormatType};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use std::{io::Read, path::Path};

use self::{blake3::Blake3, ed25519_signer::Ed25519Signer, ed25519_verifier::Ed25519Verifier};

pub fn sign_text(text: &str, key: &str, format: TextSignFormatType) -> Result<String> {
    let mut reader = get_reader_from_path(text)?;
    let signed = match format {
        TextSignFormatType::Blake3 => {
            let blake3 = Blake3::load(key)?;
            blake3.sign(&mut reader)?
        }
        TextSignFormatType::Ed25519 => {
            let ed25519 = Ed25519Signer::load(key)?;
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
            let ed25519 = Ed25519Verifier::load(key)?;
            ed25519.verify(&mut reader, &sig)
        }
    }
}

pub fn generate_key(format: TextSignFormatType) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormatType::Blake3 => Blake3::generate(),
        TextSignFormatType::Ed25519 => Ed25519Signer::generate(),
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
        let sk = Ed25519Signer::load("fixtures/process_text/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/process_text/ed25519.pk")?;
        let data = b"hello world!";
        let sign = sk.sign(&mut &data[..])?;
        let verify = pk.verify(&mut &data[..], &sign)?;
        assert!(verify);
        Ok(())
    }
}

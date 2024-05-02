use crate::Base64FormatType;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};

pub fn encode_base64(input: &str, format: Base64FormatType) -> Result<String> {
    let bytes = input.as_bytes().to_vec();

    let encode_str = match format {
        Base64FormatType::UrlSafe => URL_SAFE_NO_PAD.encode(bytes),
        Base64FormatType::Standard => STANDARD.encode(bytes),
    };

    Ok(encode_str)
}

pub fn decode_base64(input: &str, format: Base64FormatType) -> Result<String> {
    let decode_str = match format {
        Base64FormatType::UrlSafe => URL_SAFE_NO_PAD.decode(input),
        Base64FormatType::Standard => STANDARD.decode(input),
    }?;

    Ok(String::from_utf8(decode_str)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_base64_url_safe() {
        let input = "aGVsbG8gd29ybGQ";
        let format = Base64FormatType::UrlSafe;
        let result = decode_base64(input, format).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_decode_base64_standard() {
        let input = "aGVsbG8gd29ybGQ=";
        let format = Base64FormatType::Standard;
        let result = decode_base64(input, format).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_encode_base64_url_safe() {
        let input = "hello world";
        let format = Base64FormatType::UrlSafe;
        let result = encode_base64(input, format).unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ");
    }

    #[test]
    fn test_encode_base64_standard() {
        let input = "hello world";
        let format = Base64FormatType::Standard;
        let result = encode_base64(input, format).unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ=");
    }
}

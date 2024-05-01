use anyhow::Result;
use rand::{prelude::SliceRandom, thread_rng};

const UPPER_CASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER_CASE: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"23456789";
const SYMBOL: &[u8] = b"!@#$%&*=";

pub fn gen_pass(
    length: u8,
    no_uppercase: bool,
    no_lowercase: bool,
    no_number: bool,
    no_symbol: bool,
) -> Result<String> {
    let mut rng = thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if !no_uppercase {
        chars.extend_from_slice(UPPER_CASE);
        password.push(*UPPER_CASE.choose(&mut rng).unwrap())
    }

    if !no_lowercase {
        chars.extend_from_slice(LOWER_CASE);
        password.push(*LOWER_CASE.choose(&mut rng).unwrap())
    }

    if !no_number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap())
    }

    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap())
    }

    for _ in 0..(length - password.len() as u8) {
        let word = chars.choose(&mut rng).unwrap();
        password.push(*word)
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;

    Ok(password)
}

// 导入依赖
#[cfg(test)]
mod tests {
    use super::*;

    // 测试函数
    #[test]
    fn test_handle_gen_pass_command() {
        // 测试生成密码的长度是否正确
        let result = gen_pass(8, false, false, false, false);
        assert_eq!(result.unwrap().len(), 8);

        // 测试是否能够生成包含大写字母的密码
        let result = gen_pass(8, false, true, true, true);
        assert!(result
            .unwrap()
            .chars()
            .any(|c| String::from_utf8_lossy(UPPER_CASE).into_owned().contains(c)));

        // 测试是否能够生成包含小写字母的密码
        let result = gen_pass(8, true, false, true, true);
        assert!(result
            .unwrap()
            .chars()
            .any(|c| String::from_utf8_lossy(LOWER_CASE).into_owned().contains(c)));

        // 测试是否能够生成包含数字的密码
        let result = gen_pass(8, true, true, false, true);
        assert!(result
            .unwrap()
            .chars()
            .any(|c| String::from_utf8_lossy(NUMBER).into_owned().contains(c)));

        // 测试是否能够生成包含符号的密码
        let result = gen_pass(8, true, true, true, false);
        assert!(result
            .unwrap()
            .chars()
            .any(|c| String::from_utf8_lossy(SYMBOL).into_owned().contains(c)));
    }
}

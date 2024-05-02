use crate::{generate_key, sign_text, utils::verify_file, verify_dir, verify_text, CmdExecutor};
use anyhow::Result;
use clap::Parser;
use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "对文件或文本进行签名")]
    Sign(TextSignOption),
    #[command(about = "验证文本或者文件签名")]
    Verify(TextVerifyOption),
    #[command(about = "生成签名密钥")]
    Generate(TextGenerateOption),
}

impl CmdExecutor for TextSubCommand {
    async fn execute(&self) -> Result<()> {
        match self {
            TextSubCommand::Sign(opt) => opt.execute().await,
            TextSubCommand::Verify(opt) => opt.execute().await,
            TextSubCommand::Generate(opt) => opt.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct TextSignOption {
    /// 需要签名的文件路径,“-”为从标准输入读取
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    /// 签名的密钥文件路径
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,

    /// 签名的算法
    #[arg(short, long, value_parser=parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormatType,
}

impl CmdExecutor for TextSignOption {
    async fn execute(&self) -> Result<()> {
        let sign_text = sign_text(&self.input, &self.key, self.format)?;
        println!("{}", sign_text);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormatType {
    Blake3,
    Ed25519,
}

impl From<TextSignFormatType> for &'static str {
    fn from(value: TextSignFormatType) -> Self {
        match value {
            TextSignFormatType::Blake3 => "blake3",
            TextSignFormatType::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormatType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormatType::Blake3),
            "ed25519" => Ok(TextSignFormatType::Ed25519),
            _ => Err(format!("Invalid sign format type: {}", s)),
        }
    }
}

impl Display for TextSignFormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

fn parse_text_sign_format(s: &str) -> Result<TextSignFormatType, String> {
    s.parse()
}

#[derive(Debug, Parser)]
pub struct TextVerifyOption {
    /// 需要验证文件内容路径,“-”为从标准输入读取
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    /// 签名的密钥文件路径
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,

    /// 签名路径
    #[arg(short, long, value_parser=verify_file)]
    pub sin: String,

    /// 签名的算法
    #[arg(short, long, value_parser=parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormatType,
}

impl CmdExecutor for TextVerifyOption {
    async fn execute(&self) -> Result<()> {
        let verify_text = verify_text(&self.input, &self.key, &self.sin, self.format)?;
        println!("{}", verify_text);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextGenerateOption {
    #[arg(long, value_parser=parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormatType,

    #[arg(short, long, value_parser=verify_dir)]
    pub output: PathBuf,
}

impl CmdExecutor for TextGenerateOption {
    async fn execute(&self) -> Result<()> {
        let key = generate_key(self.format)?;
        match self.format {
            TextSignFormatType::Blake3 => {
                let path = self.output.join("blake3.txt");
                fs::write(path, &key[0])?;
            }
            TextSignFormatType::Ed25519 => {
                let sk_path = self.output.join("ed25519.sk");
                let pk_path = self.output.join("ed25519.pk");
                fs::write(sk_path, &key[0])?;
                fs::write(pk_path, &key[1])?;
            }
        }
        Ok(())
    }
}

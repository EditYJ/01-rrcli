use crate::{
    get_string_from_path,
    process::{decode_base64, encode_base64},
    utils::verify_file,
    CmdExecutor,
};
use clap::{Parser, Subcommand};
use std::{fmt::Display, str::FromStr};

#[derive(Subcommand)]
pub enum Base64SubCommand {
    Encode(Base64EncodeOptions),
    Decode(Base64DecodeOptions),
}

impl CmdExecutor for Base64SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOptions {
    /// 需要解析的base64文件路径,“-”为从标准输入读取
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    input: String,

    /// 解析的base64模式
    #[arg(short, long, value_parser=verify_base64_format, default_value = "urlsafe")]
    format: Base64FormatType,
}

impl CmdExecutor for Base64EncodeOptions {
    async fn execute(&self) -> anyhow::Result<()> {
        let content = get_string_from_path(&self.input)?;
        let encode_base64 = encode_base64(content.as_str(), self.format)?;

        println!("{}", encode_base64);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOptions {
    /// 需要解析的base64文件路径,“-”为从标准输入读取
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    input: String,

    /// 解析的base64模式
    #[arg(short, long, value_parser=verify_base64_format, default_value = "urlsafe")]
    format: Base64FormatType,
}

impl CmdExecutor for Base64DecodeOptions {
    async fn execute(&self) -> anyhow::Result<()> {
        let content = get_string_from_path(&self.input)?;
        let decode_base64 = decode_base64(content.as_str(), self.format)?;

        println!("{}", decode_base64);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64FormatType {
    UrlSafe,
    Standard,
}

impl From<Base64FormatType> for &'static str {
    fn from(value: Base64FormatType) -> Self {
        match value {
            Base64FormatType::UrlSafe => "urlsafe",
            Base64FormatType::Standard => "standard",
        }
    }
}

impl FromStr for Base64FormatType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "urlsafe" => Ok(Base64FormatType::UrlSafe),
            "standard" => Ok(Base64FormatType::Standard),
            _ => Err(format!("Invalid base64 format type: {}", s)),
        }
    }
}

impl Display for Base64FormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Into::<&'static str>::into(*self))
    }
}

fn verify_base64_format(s: &str) -> Result<Base64FormatType, String> {
    s.parse()
}

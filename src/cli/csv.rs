use super::CmdExecutor;
use crate::convert_csv_in_file;
use anyhow::Result;
use clap::Parser;
use std::{fmt::Display, path::Path, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum CsvFormatType {
    Json,
    Yaml,
    Toml,
}

// 实现 From<CsvFormatType> for &'static str
// From和Into用于值到值的转换，From是Into的逆运算。
// 优先考虑实现From，因为当From<T>实现后，由于标准库的泛型实现，Into<U>也会自动得到实现。
// From不应失败，设计用于无损转换；如果可能失败或不是无损转换，应使用TryFrom。
// 这里通过实现From，可以使用.into()方法直接将CsvFormatType枚举转换为静态字符串引用。
impl From<CsvFormatType> for &'static str {
    fn from(value: CsvFormatType) -> Self {
        match value {
            CsvFormatType::Json => "json",
            CsvFormatType::Yaml => "yaml",
            CsvFormatType::Toml => "toml",
        }
    }
}

// 实现 FromStr
// FromStr用于字符串到值的转换。
impl FromStr for CsvFormatType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(CsvFormatType::Json),
            "yaml" => Ok(CsvFormatType::Yaml),
            "toml" => Ok(CsvFormatType::Toml),
            _ => Err("不支持的文件格式"),
        }
    }
}

impl Display for CsvFormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

fn parse_csv_format_value(s: &str) -> Result<CsvFormatType, &'static str> {
    s.parse()
}

fn verify_file(s: &str) -> Result<String, String> {
    if Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(format!("指定文件不存在: {}", s))
    }
}

#[derive(Debug, Parser)]
pub struct CsvOptions {
    /// 需要解析的csv文件路径
    #[arg(short, long, value_parser=verify_file)]
    pub input: String,

    /// 输出文件路径，默认当前目录output.{format}
    #[arg(short, long)]
    pub output: Option<String>,

    /// 输出文件格式，默认json
    #[arg(short, long, value_parser=parse_csv_format_value, default_value = "json")]
    pub format: CsvFormatType,
}

impl CmdExecutor for CsvOptions {
    async fn execute(&self) -> Result<()> {
        let output = if let Some(output) = &self.output {
            output.clone()
        } else {
            format!("output.{}", self.format)
        };
        convert_csv_in_file(self.input.clone(), output, self.format)?;

        Ok(())
    }
}

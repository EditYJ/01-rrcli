mod csv;

use clap::{command, Parser, Subcommand};
pub use csv::{CsvFormatType, CsvOptions};

#[derive(Subcommand)]
pub enum RCliCommand {
    #[command(about = "转换csv文件内容到json、yaml、toml")]
    Csv(CsvOptions),
}

#[derive(Parser)]
#[command(version, about, about = "======= 🔥欢迎使用程序员常用工具箱🔥 =======")]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: RCliCommand,
}

mod csv;

use anyhow::Result;
use clap::{command, Parser, Subcommand};
pub use csv::{CsvFormatType, CsvOptions};

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(&self) -> Result<()>;
}

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

impl CmdExecutor for Cli {
    async fn execute(&self) -> Result<()> {
        match &self.subcommand {
            RCliCommand::Csv(options) => options.execute().await,
        }
    }
}

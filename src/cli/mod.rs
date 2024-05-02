mod base64;
mod csv;
mod gen_pass;

use anyhow::Result;
pub use base64::Base64FormatType;
use clap::{command, Parser, Subcommand};
pub use csv::{CsvFormatType, CsvOptions};

use self::{base64::Base64SubCommand, gen_pass::GenPassOptions};

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(&self) -> Result<()>;
}

#[derive(Subcommand)]
pub enum RCliCommand {
    #[command(about = "转换csv文件内容到json、yaml、toml")]
    Csv(CsvOptions),
    #[command(name = "genpass", about = "转换csv文件内容到json、yaml、toml")]
    GenPass(GenPassOptions),
    #[command(subcommand)]
    Base64(Base64SubCommand),
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
            RCliCommand::Csv(opt) => opt.execute().await,
            RCliCommand::GenPass(opt) => opt.execute().await,
            RCliCommand::Base64(sub_cmd) => sub_cmd.execute().await,
        }
    }
}

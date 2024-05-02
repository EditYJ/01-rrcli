use std::path::PathBuf;

use clap::Parser;

use crate::{http_serve, verify_dir, CmdExecutor};

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "启动一个http服务")]
    Serve(HttpServeOption),
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(sub_cmd) => sub_cmd.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct HttpServeOption {
    #[arg(short, long, value_parser=verify_dir, default_value = ".")]
    pub root: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOption {
    async fn execute(&self) -> anyhow::Result<()> {
        http_serve(self.root.clone(), self.port).await
    }
}

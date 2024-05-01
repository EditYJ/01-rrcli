use clap::Parser;

use crate::{gen_pass, CmdExecutor};

#[derive(Debug, Parser)]
pub struct GenPassOptions {
    /// 密码长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// 是否不包含数字  [default: false]
    #[arg(long, default_value_t = false)]
    pub no_numbers: bool,

    /// 是否不包含符号  [default: false]
    #[arg(long, default_value_t = false)]
    pub no_symbols: bool,

    /// 是否不包含大写字母  [default: false]
    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    /// 是否不包含小写字母  [default: false]
    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool,
}

impl CmdExecutor for GenPassOptions {
    async fn execute(&self) -> anyhow::Result<()> {
        let gen_pass = gen_pass(
            self.length,
            self.no_uppercase,
            self.no_lowercase,
            self.no_numbers,
            self.no_symbols,
        )?;

        println!("{}", gen_pass);

        Ok(())
    }
}

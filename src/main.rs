use anyhow::Result;
use clap::Parser;
use rrcli::{convert_csv_in_file, Cli, RCliCommand};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        RCliCommand::Csv(options) => {
            let output = if let Some(output) = options.output {
                output.clone()
            } else {
                format!("output.{}", options.format)
            };
            convert_csv_in_file(options.input, output, options.format)
        }
    }?;

    Ok(())
}

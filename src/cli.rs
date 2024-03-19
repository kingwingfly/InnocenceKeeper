use crate::actions::{check, run};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Check,
    Run,
}

impl Cli {
    pub fn run() -> Result<()> {
        let args = Cli::parse();
        match args.subcmd {
            SubCommand::Check => check()?,
            SubCommand::Run => run()?,
        }
        Ok(())
    }
}

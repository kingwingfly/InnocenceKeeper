use crate::app::App;
use anyhow::Result;
use clap::{CommandFactory as _, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Check the existence of the files
    Check,
    /// Run the removal of the files
    Run,
    /// Add a file to the list
    Add { path: PathBuf },
    /// Remove a file from the list
    Remove { path: PathBuf },
    /// Generate completion script
    Completion { shell: clap_complete::Shell },
}

impl Cli {
    pub fn run() -> Result<()> {
        let args = Cli::parse();
        let mut app = App::new();
        match args.subcmd {
            SubCommand::Check => app.check()?,
            SubCommand::Run => app.run(),
            SubCommand::Add { path } => app.add(path)?,
            SubCommand::Remove { path } => app.remove(path)?,
            SubCommand::Completion { shell } => {
                let mut cmd = Cli::command();
                clap_complete::generate(shell, &mut cmd, "InnocenceKeeper", &mut std::io::stdout());
            }
        }
        Ok(())
    }
}

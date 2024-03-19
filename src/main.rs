pub(crate) mod app;
mod cli;
pub(crate) mod config;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
    Cli::run()?;
    Ok(())
}

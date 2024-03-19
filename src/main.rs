pub(crate) mod actions;
mod cli;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
    Cli::run()?;
    Ok(())
}

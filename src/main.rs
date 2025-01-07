mod cli;
mod prompt;
mod utils;

use cli::Cli;

// Error handling
use miette::Result;

fn main() -> Result<()> {
    Cli::run()?;
    Ok(())
}

mod cli;
mod prompt;

use cli::Cli;

// Error handling
use miette::Result;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    Cli::run()?;
    #[cfg(not(debug_assertions))]
    if is_ssh_session() {
        Cli::run()?;
    }
    Ok(())
}

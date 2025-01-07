mod cli;
mod prompt;

use cli::Cli;
use std::env;

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

fn is_ssh_session() -> bool {
    env::var("SSH_TTY").is_ok()
}

use super::prompt::Prompt;
// Error Handling
use miette::{IntoDiagnostic, Result};

use clap::{Args, Parser, Subcommand, ValueEnum, ValueHint};
// use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
    // #[command(flatten)]
    // pub verbose: Verbosity,
}

#[derive(Debug, Subcommand, Clone, Eq, PartialEq)]
pub enum Commands {
    HostChallenge(AskArgs),
    Ask(AskArgs),
}

#[derive(Debug, Args, Clone, Eq, PartialEq)]
pub struct AskArgs {
    cmd: String,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Cli::parse();
        match cli.commands {
            Commands::Ask(args) => {
                let prompt = Prompt::builder().cmd(args.cmd).build();
                prompt.display_ask()?;
            }
            Commands::HostChallenge(args) => {
                let prompt = Prompt::builder().cmd(args.cmd).build();
                prompt.display_host_challenge()?;
            }
        };
        Ok(())
    }
}

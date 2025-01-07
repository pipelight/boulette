use super::prompt::Prompt;
use owo_colors::colors::*;
use owo_colors::OwoColorize;

use clap::FromArgMatches;
use clap::{Args, Command, Parser, Subcommand, ValueEnum, ValueHint};
// use clap_verbosity_flag::{InfoLevel, Verbosity};

// Error Handling
use miette::Result;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub cmd: String,

    // #[arg(long, global = true , num_args=0..1, require_equals= true, default_missing_value= "false" , default_value = "false")]
    #[arg(long, global = true, required = false)]
    pub ssh_only: Option<bool>,

    #[arg(
        long,
        global = true,
        default_missing_value = "ask",
        default_value = "ask"
    )]
    pub challenge: Option<Challenges>,
}

#[derive(Debug, Clone, Eq, PartialEq, ValueEnum)]
pub enum Challenges {
    Hostname,
    Ask,
}

impl Cli {
    pub fn run() -> Result<()> {
        let mut cli = Command::new("boulette");
        cli = Cli::augment_args(cli);
        let example = format!(
            "{}\n{}",
            "Example:".underline().bold(),
            "alias shutdown = boulette ask --ssh-only --cmd shutdown"
        );
        cli = cli.mut_arg("ssh_only", |e| {
            e.num_args(0..=1)
                .require_equals(true)
                .default_missing_value("true")
                .default_value("false")
        });
        cli = cli.after_help(&example);

        let matches = cli.get_matches();
        let cli = Cli::from_arg_matches(&matches)
            .map_err(|err| err.exit())
            .unwrap();

        if let Some(challenge) = cli.challenge {
            match challenge {
                Challenges::Ask => {
                    let prompt = Prompt::builder().cmd(cli.cmd).build();
                    prompt.display_ask()?;
                }
                Challenges::Hostname => {
                    let prompt = Prompt::builder().cmd(cli.cmd).build();
                    prompt.display_host_challenge()?;
                }
            };
        }
        Ok(())
    }
}

use super::prompt::Prompt;
use super::utils::is_ssh_session;

use owo_colors::colors::*;
use owo_colors::OwoColorize;
use std::env;

use clap::FromArgMatches;
use clap::{Args, Command, Parser, Subcommand, ValueEnum, ValueHint};
// use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::process;

// Error Handling
use miette::{IntoDiagnostic, Result};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub cmd: String,

    #[arg(
        long,
        global = true,
        required = false,
        default_missing_value = "false",
        default_value = "false"
    )]
    pub ssh_only: Option<bool>,

    #[arg(
        long,
        global = true,
        required = false,
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

        if cli.ssh_only.unwrap() && !is_ssh_session() {
        } else {
            let prompt = Prompt::builder().cmd(cli.cmd.clone()).build();
            let res = match cli.challenge.unwrap() {
                Challenges::Ask => prompt.display_ask(),
                Challenges::Hostname => prompt.display_host_challenge(),
            };
            if res.is_err() {
                return Ok(());
            }
        }

        let default_shell = env::var("SHELL").into_diagnostic()?;
        let mut p = process::Command::new(default_shell);
        p.arg("-c").arg(&cli.cmd);
        p.spawn().into_diagnostic()?;
        Ok(())
    }
}

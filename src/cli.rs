use super::prompt::Prompt;
use super::utils::{get_spawning_shell, is_ssh_session};

use owo_colors::OwoColorize;

use std::process;

use clap::FromArgMatches;
use clap::{Args, Command, Parser, ValueEnum, ValueHint};
// use clap_verbosity_flag::{InfoLevel, Verbosity};

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
        num_args= 0..1,
        default_missing_value = "true",
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
    Numbers,
    Chars,
}

impl Cli {
    pub fn run() -> Result<()> {
        let mut cli = Command::new("boulette");
        cli = Cli::augment_args(cli);
        let example = format!(
            "{}\n{}",
            "Example:".underline().bold(),
            "alias off = boulette 'shutdown -h now' --ssh-only"
        );
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
                Challenges::Numbers => prompt.display_numbers_challenge(),
                Challenges::Chars => prompt.display_chars_challenge(),
            };
            if res.is_err() {
                return Ok(());
            }
        }

        // Use a subshell
        let shell = get_spawning_shell()?;
        let args: Vec<&str> = cli.cmd.split(' ').collect();
        let mut p = process::Command::new(&shell.name);
        p.arg("-c");
        if !args.is_empty() {
            p.arg(cli.cmd);
        }

        p.stdin(process::Stdio::null())
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .output()
            .into_diagnostic()?;
        Ok(())
    }
}

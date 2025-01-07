// Colors
use owo_colors::colors::*;
use owo_colors::OwoColorize;

use bon::{builder, Builder};
use std::env;
use std::process::{exit, ExitCode};

use gethostname::gethostname;

use inquire::{
    validator::{StringValidator, Validation},
    Confirm, Text,
};

// Error handling
use miette::{IntoDiagnostic, Result};
use std::error::Error;

fn is_ssh_session() -> bool {
    env::var("SSH_TTY").is_ok()
}

fn host_validator(input: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    let hostname = gethostname();
    if input == hostname {
        Ok(Validation::Valid)
    } else {
        let message = "Input do not match host name";
        Ok(Validation::Invalid(message.into()))
    }
}

#[derive(Debug, Default, Builder)]
pub struct Prompt {
    cmd: String,
}

impl Prompt {
    fn display_warning() -> Result<()> {
        let user = env::var("USER").into_diagnostic()?;
        let hostname = format!("@{}", gethostname().to_str().unwrap());
        println!(
            "{}: {} {}{} -> {}",
            "Warning".yellow().bold(),
            "(molly-breaks)",
            "You are on a remote ssh session".bold(),
            user.yellow().bold(),
            hostname.green().bold()
        );
        Ok(())
    }
    fn make_prompt_text(&self) -> Result<String> {
        let hostname = format!("@{}", gethostname().to_str().unwrap());
        // Shorten cmd
        let short_cmd: String = if self.cmd.len() > 15 {
            self.cmd.clone().drain(..15).collect()
        } else {
            self.cmd.clone()
        };
        let text = format!(
            "Do you really want to execute \"{}\" on {}\n",
            short_cmd.purple(),
            hostname.green().bold()
        );
        Ok(text)
    }

    pub fn display_ask(&self) -> Result<()> {
        Self::display_warning()?;
        let text = self.make_prompt_text()?;

        let res = Confirm::new(&text)
            .with_help_message("type y/n to confirm/abort")
            .prompt();

        match res {
            Ok(status) => match status {
                true => {
                    println!("{}", "Resuming!".green().bold());
                }
                false => {
                    println!("{}", "Aborting!".red().bold());
                }
            },
            Err(err) => {}
        }
        Ok(())
    }

    pub fn display_host_challenge(&self) -> Result<()> {
        Self::display_warning()?;
        let text = self.make_prompt_text()?;

        let status = Text::new(&text)
            // .with_validator(host_validator)
            .with_help_message("type host name to confirm")
            .prompt();

        match status {
            Ok(status) => {
                if status == gethostname().to_str().unwrap() {
                    println!("{}", "Resuming!".green().bold());
                } else {
                    let message = format!(
                        "{}\n{}",
                        "Provided input do not match host name.".red(),
                        "Aborting!".red().bold()
                    );
                    println!("{}", message);
                }
            }
            Err(err) => {}
        };
        Ok(())
    }
}

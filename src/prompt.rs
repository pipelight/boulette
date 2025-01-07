// Colors
use owo_colors::colors::*;
use owo_colors::OwoColorize;

use bon::Builder;
use std::env;

use gethostname::gethostname;

use inquire::{validator::Validation, Confirm, Text};

// Error handling
use miette::{Error, IntoDiagnostic, Result};

#[derive(Debug, Default, Builder)]
pub struct Prompt {
    cmd: String,
}

impl Prompt {
    fn display_warning() -> Result<()> {
        let user = env::var("USER").into_diagnostic()?;
        let hostname = format!("@{}", gethostname().to_str().unwrap());
        println!(
            "{}: {} {} -> {}{}",
            "Warning".yellow().bold(),
            "(boulette)",
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
            Ok(status) => {
                match status {
                    true => {
                        println!("{}", "Resuming!".green().bold());
                    }
                    false => {
                        println!("{}", "Aborting!".red().bold());
                    }
                }
                return Ok(());
            }
            Err(err) => Err(Error::msg(err)),
        }
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
                return Ok(());
            }
            Err(err) => Err(Error::msg(err)),
        }
    }
}

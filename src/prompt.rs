// Colors
use super::utils::is_ssh_session;

use owo_colors::OwoColorize;
use std::iter::repeat_with;

use bon::Builder;
use std::env;

use gethostname::gethostname;

use inquire::{Confirm, Text};

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
        if is_ssh_session() {
            println!(
                "{}: {} {} -> {}{}",
                "Warning".yellow().bold(),
                "(boulette)",
                "You are on a remote ssh session".bold(),
                user.yellow().bold(),
                hostname.green().bold()
            );
        } else {
            println!(
                "{}: {} {} -> {}{}",
                "Warning".yellow().bold(),
                "(boulette)",
                "You are on a local terminal".bold(),
                user.yellow().bold(),
                hostname.green().bold()
            );
        }
        Ok(())
    }
    fn make_prompt_text(&self) -> Result<String> {
        let hostname = format!("@{}", gethostname().to_str().unwrap());
        // Shorten cmd
        let short_cmd: String;
        if self.cmd.len() > 30 {
            let s: String = self.cmd.clone().drain(..30).collect();
            short_cmd = format!("{}...", s);
        } else {
            short_cmd = self.cmd.clone()
        };
        let text = format!(
            "Do you really want to execute `{}` on {}\n",
            short_cmd.purple(),
            hostname.green().bold()
        );
        Ok(text)
    }

    /*
     * Print a prompt with a y/n confirmation input
     */
    pub fn display_ask(&self) -> Result<()> {
        Self::display_warning()?;
        let text = self.make_prompt_text()?;

        let res = Confirm::new(&text)
            .with_help_message("Type y/n to confirm/abort.")
            .prompt();

        match res {
            Ok(status) => match status {
                true => {
                    let message = "Resuming!";
                    println!("{}", message.green().bold());
                    return Ok(());
                }
                false => {
                    let message = "Aborting!";
                    println!("{}", message.red().bold());
                    return Err(Error::msg(message));
                }
            },
            Err(err) => Err(Error::msg(err)),
        }
    }

    pub fn display_host_challenge(&self) -> Result<()> {
        Self::display_warning()?;
        let text = self.make_prompt_text()?;

        let status = Text::new(&text)
            .with_help_message("Type hostname to confirm.")
            .prompt();

        match status {
            Ok(status) => {
                if status == gethostname().to_str().unwrap() {
                    println!("{}", "Resuming!".green().bold());
                    Ok(())
                } else {
                    let message = format!(
                        "{}\n{}",
                        "Provided input do not match hostname.".red(),
                        "Aborting!".red().bold()
                    );
                    println!("{}", message);
                    return Err(Error::msg(message));
                }
            }
            Err(err) => Err(Error::msg(err)),
        }
    }

    pub fn display_numbers_challenge(&self) -> Result<()> {
        Self::display_warning()?;
        let text = self.make_prompt_text()?;

        let num: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(6).collect();
        let num: String = num.iter().map(|e| e.to_string()).collect();

        let help = format!("Type these numbers to confirm: {}", num);
        let status = Text::new(&text).with_help_message(&help).prompt();
        match status {
            Ok(status) => {
                if status == num {
                    println!("{}", "Resuming!".green().bold());
                    Ok(())
                } else {
                    let message = format!(
                        "{}\n{}",
                        "Numbers do not match.".red(),
                        "Aborting!".red().bold()
                    );
                    println!("{}", message);
                    return Err(Error::msg(message));
                }
            }
            Err(err) => Err(Error::msg(err)),
        }
    }

    pub fn display_chars_challenge(&self) -> Result<()> {
        Self::display_warning()?;
        let text = self.make_prompt_text()?;

        let chars: String = repeat_with(|| fastrand::char('a'..='z')).take(6).collect();

        let help = format!("Type this string to confirm: {}", chars);
        let status = Text::new(&text).with_help_message(&help).prompt();
        match status {
            Ok(status) => {
                if status == chars {
                    println!("{}", "Resuming!".green().bold());
                    Ok(())
                } else {
                    let message = format!(
                        "{}\n{}",
                        "Numbers do not match.".red(),
                        "Aborting!".red().bold()
                    );
                    println!("{}", message);
                    return Err(Error::msg(message));
                }
            }
            Err(err) => Err(Error::msg(err)),
        }
    }
}

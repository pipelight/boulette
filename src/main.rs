// Colors
use owo_colors::colors::*;
use owo_colors::OwoColorize;

use std::env;
use std::process::{exit, ExitCode};

use gethostname::gethostname;

use inquire::{
    validator::{StringValidator, Validation},
    Text,
};

// Error handling
use miette::{IntoDiagnostic, Result};
use std::error::Error;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    display_confirmation_dialog()?;
    #[cfg(not(debug_assertions))]
    if is_ssh_session() {
        display_confirmation_dialog()?;
    }
    Ok(())
}

fn validator(input: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    if input == gethostname() {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid(
            "Input doesn't match with hostname! Aborting!".into(),
        ))
    }
}

fn is_ssh_session() -> bool {
    env::var("SSH_TTY").is_ok()
}

fn display_confirmation_dialog() -> Result<()> {
    let user = env::var("USER").into_diagnostic()?;
    let hostname = format!("@{}", gethostname().to_str().unwrap());

    println!(
        "{}: {} {}{}{}",
        "Warning".yellow().bold(),
        "(molly-breaks)",
        "You are on a remote ssh session -> ".bold(),
        user.yellow().bold(),
        hostname.green().bold()
    );
    println!("Do you realy want to shutdown {}?", hostname.green().bold());

    let text = format!("Type hostname to confirm:");
    let status = Text::new(&text).with_validator(validator).prompt();

    match status {
        Ok(status) => {
            println!("Executing command!");
        }
        Err(err) => {
            println!("Aborting!");
        }
    };
    Ok(())
}

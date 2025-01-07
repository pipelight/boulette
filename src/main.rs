use gethostname::gethostname;
use inquire::{
    validator::{StringValidator, Validation},
    Text,
};

// Error handling
use miette::{IntoDiagnostic, Result};
use std::error::Error;

fn main() {
    display_confirmation_dialog().unwrap();
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
fn display_confirmation_dialog() -> Result<()> {
    let text = format!(
        "You are on a remote host {:?} through ssh. Do you realy want to shutdown?",
        gethostname()
    );
    let status = Text::new(&text).with_validator(validator).prompt();

    match status {
        Ok(status) => println!("Executing command!"),
        Err(err) => println!("Aborting!"),
    };

    Ok(())
}

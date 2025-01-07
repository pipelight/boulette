use std::env;

pub fn is_ssh_session() -> bool {
    env::var("SSH_TTY").is_ok()
}

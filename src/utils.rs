use miette::{Error, Result};
use std::env;
use std::process;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System, UpdateKind};

pub fn is_ssh_session() -> bool {
    if env::var("SSH_TTY").is_ok() {
        return true;
    } else {
        return is_nested_ssh_session();
    }
}

/*
* Is the current boulette process grand-child of an ssh process.
*/
pub fn is_nested_ssh_session() -> bool {
    // Get system process list
    let s = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::nothing().with_cmd(UpdateKind::Never)),
    );
    return is_parent_ssh(&process::id(), &s);
}

/*
* Recursive function to get a process parent until a parent
* that is a sshd process is found.
*/
pub fn is_parent_ssh(pid: &u32, s: &System) -> bool {
    if let Some(process) = s.process(Pid::from(pid.to_owned() as usize)) {
        if let Some(parent_pid) = process.parent() {
            if let Some(parent) = s.process(parent_pid) {
                if parent.name().to_str().unwrap().contains("sshd") {
                    return true;
                } else {
                    return is_parent_ssh(&parent_pid.as_u32(), &s);
                }
            }
        }
    }
    return false;
}

#[derive(Debug, Clone)]
pub struct FastProc {
    pub name: String,
}
/*
* Get the current process spawning shell
*/
pub fn get_spawning_shell() -> Result<FastProc> {
    // Get system process list
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(
            ProcessRefreshKind::nothing()
                .with_cmd(UpdateKind::Never)
                .with_environ(UpdateKind::Never),
        ),
    );
    return get_parent_spawning_shell(&process::id(), &s);
}

/*
* Recursive function to get a process parent
*/
pub fn get_parent_spawning_shell(pid: &u32, s: &System) -> Result<FastProc> {
    let shells = vec!["sh", "bash", "zsh", "fish", "nu"];

    if let Some(process) = s.process(Pid::from(pid.to_owned() as usize)) {
        if let Some(parent_pid) = process.parent() {
            if let Some(parent) = s.process(parent_pid) {
                let name = parent.name().to_str().unwrap();
                if shells.contains(&name) {
                    return Ok(FastProc {
                        name: name.to_owned(),
                    });
                } else {
                    return get_parent_spawning_shell(&parent_pid.as_u32(), &s);
                }
            }
        }
    }
    let message = format!("Couldn't get the spawning shell");
    Err(Error::msg(message))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn _is_ssh_session() -> Result<()> {
        is_nested_ssh_session();
        Ok(())
    }
    #[test]
    fn _get_spawning_shell() -> Result<()> {
        let shell = get_spawning_shell()?;
        println!("spawning_shell = {:#?}", shell);
        Ok(())
    }
}

use miette::{IntoDiagnostic, Result};
use std::env;
use std::process::id;
use sysinfo::{Pid, Process, ProcessRefreshKind, RefreshKind, System, UpdateKind};

pub fn is_ssh_session() -> bool {
    if env::var("SSH_TTY").is_ok() {
        return true;
    } else {
        return is_nested_ssh_session();
    }
}

pub fn is_nested_ssh_session() -> bool {
    let pid = id();
    println!("{:?}", pid);

    // Get system process list
    let s = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::nothing().with_cmd(UpdateKind::Never)),
    );
    return is_parent_ssh(&pid, &s);
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn _is_ssh_session() -> Result<()> {
        is_nested_ssh_session();
        Ok(())
    }
}

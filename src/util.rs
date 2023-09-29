use anyhow::Context;
use std::process::Command;
use std::process::Stdio;

pub fn check_command_exists(command: &str) -> anyhow::Result<()> {
    let successful = Command::new("pwsh")
        .arg("-Command")
        .arg(format!("Get-Command {}", command))
        .stdout(Stdio::null())
        .spawn()
        .context("PowerShell is not installed")?
        .wait()?
        .success();
    if !successful {
        anyhow::bail!("Command {} does not exist", command);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_exists() {
        check_command_exists("rustc").unwrap();
    }

    #[test]
    fn command_does_not_exist() {
        let result = check_command_exists("some-impossible.command");
        assert!(result.is_err());
    }
}

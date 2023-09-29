use super::Formatter;
use std::process::Command;

#[derive(Default)]
pub struct PrettierFormatter;

impl Formatter for PrettierFormatter {
    fn usage_detected(&self) -> std::io::Result<bool> {
        Ok(true)
    }

    fn run(&self) -> anyhow::Result<()> {
        // Command is run in a shell because the main executable on Windows is `prettier.ps1`
        let command = "prettier --write **/*.yaml";
        let successful = Command::new("pwsh")
            .arg("-Command")
            .arg(command)
            .spawn()?
            .wait()?
            .success();
        if !successful {
            anyhow::bail!("Failed to format code using Prettier");
        }

        Ok(())
    }
}

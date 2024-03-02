use super::Formatter;
use std::process::Command;

#[derive(Default)]
pub struct BufFormatter;

impl Formatter for BufFormatter {
    fn run(&self) -> anyhow::Result<()> {
        log::debug!("Formatting using Buf");
        let successful = Command::new("buf")
            .arg("format")
            .arg("--write")
            .spawn()?
            .wait()?
            .success();
        if !successful {
            anyhow::bail!("Failed to format Protocol Buffers schema");
        }

        Ok(())
    }

    fn get_required_commands(&self) -> Vec<&'static str> {
        vec!["buf"]
    }
}

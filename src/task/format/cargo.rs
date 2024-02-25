use super::Formatter;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default)]
pub struct CargoFormatter;

impl Formatter for CargoFormatter {
    fn detect_usage(&self) -> std::io::Result<bool> {
        PathBuf::from("./Cargo.toml").try_exists()
    }

    fn run(&self) -> anyhow::Result<()> {
        let successful = Command::new("cargo").arg("fmt").spawn()?.wait()?.success();
        if !successful {
            anyhow::bail!("Failed to format Rust code");
        }

        Ok(())
    }

    fn get_required_commands(&self) -> Vec<&'static str> {
        vec!["cargo"]
    }
}

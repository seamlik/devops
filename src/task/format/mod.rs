mod cargo;
mod file_based;

use self::cargo::CargoFormatter;
use self::file_based::FileBasedFormatter;
use super::Task;

pub struct FormatTask {
    formatters: Vec<Box<dyn Formatter>>,
}

impl Task for FormatTask {
    fn get_required_commands(&self) -> Vec<&'static str> {
        vec!["cargo", "prettier"]
    }

    fn run(&self) -> anyhow::Result<()> {
        for formatter in self.formatters.iter() {
            if formatter.detect_usage()? {
                formatter.run()?;
            }
        }
        Ok(())
    }
}

impl FormatTask {
    pub fn new(formattings: &[String]) -> Self {
        Self {
            formatters: vec![
                Box::<CargoFormatter>::default(),
                Box::new(FileBasedFormatter::new(formattings)),
            ],
        }
    }
}

trait Formatter {
    fn detect_usage(&self) -> std::io::Result<bool>;
    fn run(&self) -> anyhow::Result<()>;
    fn get_required_commands(&self) -> Vec<&'static str>;
}

use self::cargo::CargoFormatter;
use self::prettier::PrettierFormatter;

mod cargo;
mod prettier;

pub struct FormatTask {
    formatters: Vec<Box<dyn Formatter>>,
}

impl FormatTask {
    pub fn run(&self) -> anyhow::Result<()> {
        for formatter in self.formatters.iter() {
            if formatter.usage_detected()? {
                formatter.run()?;
            }
        }
        Ok(())
    }
}

impl Default for FormatTask {
    fn default() -> Self {
        Self {
            formatters: vec![
                Box::<CargoFormatter>::default(),
                Box::<PrettierFormatter>::default(),
            ],
        }
    }
}

trait Formatter {
    fn usage_detected(&self) -> std::io::Result<bool>;
    fn run(&self) -> anyhow::Result<()>;
}

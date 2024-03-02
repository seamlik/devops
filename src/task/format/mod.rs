mod buf;
mod cargo;
mod file_based;

use self::buf::BufFormatter;
use self::cargo::CargoFormatter;
use self::file_based::FileBasedFormatter;
use super::Task;
use std::collections::HashSet;

pub struct FormatTask {
    formatters: Vec<Box<dyn Formatter>>,
}

impl Task for FormatTask {
    fn get_required_commands(&self) -> Vec<&'static str> {
        self.formatters
            .iter()
            .flat_map(|fmt| fmt.get_required_commands())
            .collect()
    }

    fn run(&self) -> anyhow::Result<()> {
        for formatter in self.formatters.iter() {
            formatter.run()?;
        }
        Ok(())
    }
}

impl FormatTask {
    pub fn new(formattings: &[String]) -> anyhow::Result<Self> {
        if formattings.is_empty() {
            anyhow::bail!("Must specify at least 1 formatting");
        }

        let mut formatters: Vec<Box<dyn Formatter>> =
            vec![Box::new(FileBasedFormatter::new(formattings))];
        let formatting_set: HashSet<_> = formattings.iter().cloned().collect();
        if formatting_set.contains("rust") {
            formatters.push(Box::<CargoFormatter>::default());
        }
        if formatting_set.contains("protobuf") {
            formatters.push(Box::<BufFormatter>::default());
        }
        Ok(Self { formatters })
    }
}

trait Formatter {
    fn run(&self) -> anyhow::Result<()>;
    fn get_required_commands(&self) -> Vec<&'static str>;
}

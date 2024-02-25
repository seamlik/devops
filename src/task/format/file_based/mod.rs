mod prettier;

use self::prettier::Prettier;
use super::Formatter;
use anyhow::bail;
use std::cell::Cell;
use std::collections::HashSet;
use std::process::Command;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// Formatter that walks through a directory and operates on any acceptable files.
pub struct FileBasedFormatter {
    git_failed: Cell<bool>,
    impl_list: Vec<Box<dyn FormatterImpl>>,
}

impl FileBasedFormatter {
    pub fn new(formattings: &[String]) -> Self {
        let formatting_set: HashSet<_> = formattings.iter().cloned().collect();
        let mut impl_list: Vec<Box<dyn FormatterImpl>> = vec![];
        if formatting_set.contains("css") {
            impl_list.push(Box::new(Prettier::new(".css")));
        }
        if formatting_set.contains("html") {
            impl_list.push(Box::new(Prettier::new(".html")));
        }
        if formatting_set.contains("json") {
            impl_list.push(Box::new(Prettier::new(".json")));
        }
        if formatting_set.contains("markdown") {
            impl_list.push(Box::new(Prettier::new(".md")));
        }
        if formatting_set.contains("yaml") {
            impl_list.push(Box::new(Prettier::new(".yaml")));
            impl_list.push(Box::new(Prettier::new(".yml")));
        }
        Self {
            git_failed: Cell::new(false),
            impl_list,
        }
    }
    fn extract_path<'a>(&self, entry: &'a DirEntry) -> anyhow::Result<&'a str> {
        if let Some(path) = entry.path().to_str() {
            if !self.detect_ignorance(path) && entry.file_type().is_file() {
                Ok(path)
            } else {
                Ok("")
            }
        } else {
            bail!("File path contains invalid Unicode")
        }
    }

    fn detect_ignorance(&self, path: &str) -> bool {
        if self.git_failed.get() {
            return false;
        }
        match self.detect_git_ignorance(path) {
            Ok(ignored) => ignored,
            Err(e) => {
                log::info!("{}", e);
                log::info!("Checking file ignorance using Git is now disabled.");
                self.git_failed.set(true);
                false
            }
        }
    }

    fn detect_git_ignorance(&self, path: &str) -> anyhow::Result<bool> {
        let output = Command::new("git").arg("check-ignore").arg(path).output()?;
        if let Some(exit_code) = output.status.code() {
            match exit_code {
                0 => Ok(true),
                1 => Ok(false),
                _ => anyhow::bail!(
                    "Git was unsuccessful: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            }
        } else {
            anyhow::bail!("Git was terminated")
        }
    }

    fn format(&self, path: &str) -> anyhow::Result<()> {
        let formatter = self
            .impl_list
            .iter()
            .find(|fmt| path.ends_with(fmt.get_file_name_suffix()));
        if let Some(formatter) = formatter {
            let successful = Command::new(formatter.get_command())
                .args(formatter.build_arguments(path))
                .spawn()?
                .wait()?
                .success();
            if !successful {
                bail!("Failed to format code")
            }
        } else {
            log::debug!("No formatter registered for this file");
        }
        Ok(())
    }
}

impl Formatter for FileBasedFormatter {
    fn detect_usage(&self) -> std::io::Result<bool> {
        Ok(false)
    }

    fn run(&self) -> anyhow::Result<()> {
        for entry in WalkDir::new(".").follow_links(true) {
            match entry {
                Ok(entry) => {
                    log::debug!("Scanning {}", entry.path().display());
                    let path = self.extract_path(&entry)?;
                    self.format(path)?;
                }
                Err(e) => log::debug!("Failed while scanning: {}", e),
            }
        }
        Ok(())
    }

    fn get_required_commands(&self) -> Vec<&'static str> {
        self.impl_list
            .iter()
            .flat_map(|fmt| fmt.get_required_commands())
            .chain(std::iter::once("git"))
            .collect()
    }
}

trait FormatterImpl {
    fn get_file_name_suffix(&self) -> &'static str;
    fn get_command(&self) -> &'static str;
    fn get_required_commands(&self) -> Vec<&'static str>;
    fn build_arguments(&self, path: &str) -> Vec<String>;
}

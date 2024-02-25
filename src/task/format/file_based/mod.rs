mod prettier;

use self::prettier::Prettier;
use super::Formatter;
use anyhow::bail;
use std::cell::Cell;
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// Formatter that walks through a directory and operates on any acceptable files.
pub struct FileBasedFormatter {
    git_failed: Cell<bool>,
    impl_list: Vec<Box<dyn FormatterImpl>>,
    ignored_files: HashSet<PathBuf>,
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
            ignored_files: HashSet::from([PathBuf::from_iter([".", ".git"])]),
        }
    }
    fn detect_ignorance(&self, dir_entry: &DirEntry) -> bool {
        if self.ignored_files.contains(dir_entry.path()) {
            return true;
        }
        if self.git_failed.get() {
            return false;
        }
        match self.detect_git_ignorance(dir_entry.path()) {
            Ok(ignored) => ignored,
            Err(e) => {
                log::info!("{}", e);
                log::info!("Checking file ignorance using Git is now disabled.");
                self.git_failed.set(true);
                false
            }
        }
    }

    fn detect_git_ignorance(&self, path: &Path) -> anyhow::Result<bool> {
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

    fn format(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(formatter) = self
            .impl_list
            .iter()
            .find(|fmt| path.to_string_lossy().ends_with(fmt.get_file_name_suffix()))
        {
            log::debug!("Formatting using {}", formatter.get_name());
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
    fn run(&self) -> anyhow::Result<()> {
        for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_entry(|entry| !self.detect_ignorance(entry))
        {
            match entry {
                Ok(entry) => {
                    log::debug!("Scanning {}", entry.path().display());
                    if entry.file_type().is_file() {
                        self.format(entry.path())?;
                    }
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
    fn get_name(&self) -> &'static str;
    fn get_file_name_suffix(&self) -> &'static str;
    fn get_command(&self) -> &'static str;
    fn get_required_commands(&self) -> Vec<&'static str>;
    fn build_arguments(&self, path: &Path) -> Vec<OsString>;
}

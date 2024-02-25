use super::FormatterImpl;
use std::ffi::OsString;
use std::path::Path;

pub struct Prettier {
    file_name_suffix: &'static str,
}

impl Prettier {
    pub fn new(file_name_suffix: &'static str) -> Self {
        Self { file_name_suffix }
    }
}

impl FormatterImpl for Prettier {
    fn get_name(&self) -> &'static str {
        "Prettier"
    }
    fn get_command(&self) -> &'static str {
        "pwsh"
    }

    fn get_required_commands(&self) -> Vec<&'static str> {
        vec!["pwsh", "node", "prettier"]
    }

    fn get_file_name_suffix(&self) -> &'static str {
        self.file_name_suffix
    }

    fn build_arguments(&self, path: &Path) -> Vec<OsString> {
        // Command is run in a shell because the main executable on Windows is `prettier.ps1`
        let mut command: OsString = "prettier --write ".into();
        command.push(path);
        vec!["-Command".into(), command]
    }
}

use super::FormatterImpl;

pub struct Prettier {
    file_name_suffix: &'static str,
}

impl Prettier {
    pub fn new(file_name_suffix: &'static str) -> Self {
        Self { file_name_suffix }
    }
}

impl FormatterImpl for Prettier {
    fn get_command(&self) -> &'static str {
        "pwsh"
    }

    fn get_required_commands(&self) -> Vec<&'static str> {
        vec!["pwsh", "node", "prettier"]
    }

    fn get_file_name_suffix(&self) -> &'static str {
        self.file_name_suffix
    }

    fn build_arguments(&self, path: &str) -> Vec<String> {
        // Command is run in a shell because the main executable on Windows is `prettier.ps1`
        vec!["-Command".to_string(), format!("prettier --write {}", path)]
    }
}

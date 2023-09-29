pub mod format;
pub mod rust_code_coverage;

pub trait Task {
    fn required_commands(&self) -> Vec<&'static str>;
    fn run(&self) -> anyhow::Result<()>;
}

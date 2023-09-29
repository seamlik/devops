mod task;

use clap::Parser;
use clap::Subcommand;
use task::format::FormatTask;
use task::rust_code_coverage::RustCodeCoverageTask;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    match Cli::parse().command {
        Command::Format => FormatTask::default().run()?,
        Command::RustCodeCoverage => RustCodeCoverageTask::default().run()?,
    }
    Ok(())
}

#[derive(Parser)]
#[command(about = "Various DevOps tasks. Run only in the root directory of a project.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Formats all code.
    Format,

    /// Compiles a code coverage report for Rust code.
    RustCodeCoverage,
}

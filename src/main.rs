mod task;
mod util;

use clap::Parser;
use clap::Subcommand;
use task::format::FormatTask;
use task::rust_code_coverage::RustCodeCoverageTask;
use task::Task;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let task: Box<dyn Task> = match Cli::parse().command {
        Command::Format { formattings } => Box::new(FormatTask::new(&formattings)),
        Command::RustCodeCoverage => Box::<RustCodeCoverageTask>::default(),
    };
    for command in task.get_required_commands().into_iter() {
        crate::util::check_command_exists(command)?;
    }
    task.run()?;
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
    Format { formattings: Vec<String> },

    /// Compiles a code coverage report for Rust code.
    RustCodeCoverage,
}

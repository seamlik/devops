mod task;
mod util;

use std::collections::BTreeSet;

use clap::Parser;
use clap::Subcommand;
use task::rust_code_coverage::RustCodeCoverageTask;
use task::Task;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let task: Box<dyn Task> = match Cli::parse().command {
        Command::RustCodeCoverage => Box::<RustCodeCoverageTask>::default(),
    };

    let required_commnads: BTreeSet<_> = task.get_required_commands().into_iter().collect();
    println!("Checking if these commands exist: {:?}", required_commnads);
    required_commnads
        .into_iter()
        .try_for_each(crate::util::check_command_exists)?;

    println!("Required commands exist, now starting the task.");
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
    /// Compiles a code coverage report for Rust code.
    RustCodeCoverage,
}

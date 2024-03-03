use super::Task;
use std::path::PathBuf;
use std::process::Command;

pub struct RustCodeCoverageTask {
    target_directory_path: PathBuf,
    report_directory_path: PathBuf,
    profraw_directory_path: PathBuf,
}

impl Task for RustCodeCoverageTask {
    fn get_required_commands(&self) -> Vec<&'static str> {
        vec!["cargo", "grcov"]
    }

    fn run(&self) -> anyhow::Result<()> {
        if self.target_directory_path.try_exists()? {
            println!("Removing the existing Cargo target directory for code coverage");
            std::fs::remove_dir_all(&self.target_directory_path)?;
        }

        println!("Creating the code coverage report directory");
        std::fs::create_dir_all(&self.report_directory_path)?;

        println!("Creating the profraw directory");
        std::fs::create_dir_all(&self.profraw_directory_path)?;

        println!("Building and running tests");
        self.run_cargo_test()?;

        println!("Compiling code coverage report");
        self.run_grcov()?;

        let report_path = PathBuf::from(format!(
            "{}/html/index.html",
            self.report_directory_path.display()
        ));
        println!(
            "Rust code coverage report at {}",
            dunce::canonicalize(report_path)?.display()
        );

        Ok(())
    }
}

impl RustCodeCoverageTask {
    fn run_cargo_test(&self) -> anyhow::Result<()> {
        let profraw = format!(
            "{}/default-%p-%m.profraw",
            dunce::canonicalize(&self.profraw_directory_path)?.display()
        );
        let successful = Command::new("cargo")
            .arg("test")
            .env(
                "CARGO_TARGET_DIR",
                dunce::canonicalize(&self.target_directory_path)?,
            )
            .env("LLVM_PROFILE_FILE", profraw)
            .env("RUSTFLAGS", "--codegen instrument-coverage")
            .spawn()?
            .wait()?
            .success();
        if !successful {
            anyhow::bail!("Failed to run cargo test");
        }

        Ok(())
    }

    fn run_grcov(&self) -> anyhow::Result<()> {
        let binary_path = format!(
            "{}/debug/deps",
            dunce::canonicalize(&self.target_directory_path)?.display()
        );
        let successful = Command::new("grcov")
            .arg("--branch")
            .arg("--binary-path")
            .arg(binary_path)
            .arg("--output-type")
            .arg("html")
            .arg("--source-dir")
            .arg(".")
            .arg("--output-path")
            .arg(&self.report_directory_path)
            .arg(".")
            .spawn()?
            .wait()?
            .success();
        if !successful {
            anyhow::bail!("Failed to run cargo test");
        }

        Ok(())
    }
}

impl Default for RustCodeCoverageTask {
    fn default() -> Self {
        let target_directory_name = "target-coverage";
        let target_directory_path = PathBuf::from(format!("./{}", target_directory_name));
        let report_directory_path =
            PathBuf::from(format!("{}/coverage", target_directory_path.display()));
        let profraw_directory_path =
            PathBuf::from(format!("{}/profraw", target_directory_path.display()));
        Self {
            target_directory_path,
            report_directory_path,
            profraw_directory_path,
        }
    }
}

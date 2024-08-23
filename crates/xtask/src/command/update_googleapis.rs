use std::path::PathBuf;
use std::process::Command;

pub fn execute() -> anyhow::Result<()> {
    let googleapis_dir = PathBuf::from("crates/xtask/googleapis");
    let status = Command::new("git")
        .args(["pull", "--rebase", "origin", "master"])
        .current_dir(&googleapis_dir)
        .status()?;
    anyhow::ensure!(status.success());
    Ok(())
}

use std::process::Command;

use crate::dirs;

pub fn execute() -> anyhow::Result<()> {
    let googleapis_dir = dirs::googleapis_dir()?;
    let status = Command::new("git")
        .args(["pull", "--rebase", "origin", "master:master"])
        .current_dir(&googleapis_dir)
        .status()?;
    anyhow::ensure!(status.success());
    Ok(())
}

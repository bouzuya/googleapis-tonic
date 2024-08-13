use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr as _;

use crate::googleapis_version::GoogleapisVersion;

pub fn execute() -> anyhow::Result<()> {
    let googleapis_dir = PathBuf::from("crates/xtask/googleapis");
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&googleapis_dir)
        .output()?;
    anyhow::ensure!(output.status.success());
    let s = String::from_utf8(output.stdout)?;
    let version = GoogleapisVersion::from_str(s.trim_end())?;
    println!("{}", version);
    Ok(())
}

use std::path::PathBuf;

use crate::googleapis_version::GoogleapisVersion;

pub fn execute() -> anyhow::Result<()> {
    let googleapis_dir = PathBuf::from("crates/xtask/googleapis");
    let version = GoogleapisVersion::load_from_googleapis_dir(&googleapis_dir)?;
    println!("{}", version);
    Ok(())
}

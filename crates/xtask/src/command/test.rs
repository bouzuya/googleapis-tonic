use std::path::PathBuf;
use std::process::Command;

use crate::state::State;

pub fn execute() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("crates");
    let generated_dir = PathBuf::from("generated");

    let state_file = crates_dir.join("xtask").join("state.json");
    let state = State::load(&state_file)?;

    for crate_name in state.publish_order() {
        let status = Command::new("cargo")
            .args(["test"])
            .current_dir(generated_dir.join(crate_name.to_string()))
            .status()?;
        if status.success() {
            println!("{} published", crate_name);
        } else {
            anyhow::bail!("{} failed", crate_name);
        }
    }
    Ok(())
}

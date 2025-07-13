use std::process::Command;

use crate::dirs;
use crate::state::State;

pub fn execute() -> anyhow::Result<()> {
    let xtask_crate_dir = dirs::xtask_crate_dir()?;
    let generated_dir = dirs::generated_dir()?;

    let state_file = xtask_crate_dir.join("state.json");
    let state = State::load(&state_file)?;

    for crate_name in state.publish_order() {
        let status = Command::new("cargo")
            .args(["test"])
            .current_dir(generated_dir.join(crate_name.to_string()))
            .status()?;
        if status.success() {
            println!("{crate_name} published");
        } else {
            anyhow::bail!("{crate_name} failed");
        }
    }
    Ok(())
}

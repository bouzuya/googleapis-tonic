use std::{fs, path::PathBuf, process::Command};

#[derive(serde::Deserialize, serde::Serialize)]
struct StateFileContent {
    next_version: String,
    publish_order: Vec<String>,
}

pub fn execute() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("crates");

    let state_file = crates_dir.join("xtask").join("state.json");
    let state = fs::read_to_string(&state_file)?;
    let StateFileContent {
        next_version: _,
        publish_order,
    } = serde_json::from_str::<StateFileContent>(&state)?;

    for crate_name in publish_order {
        let status = Command::new("cargo")
            .args(["publish", "-p", &crate_name])
            .status()?;
        if status.success() {
            println!("{} published", crate_name);
        } else {
            anyhow::bail!("{} failed", crate_name);
        }
    }

    Ok(())
}

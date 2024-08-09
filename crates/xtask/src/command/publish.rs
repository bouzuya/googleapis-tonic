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
        let crate_dir = crates_dir.join(&crate_name);
        let output = Command::new("cargo")
            .args(["publish", "--dry-run"])
            .current_dir(&crate_dir)
            .output()?;
        if output.status.success() {
            println!("{} published", crate_name);
        } else {
            std::io::Write::write_all(&mut std::io::stderr(), &output.stderr)?;
            anyhow::bail!("{} failed", crate_name);
        }
    }

    Ok(())
}

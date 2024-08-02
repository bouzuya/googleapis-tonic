use std::str::FromStr;

use anyhow::Context;

pub fn execute() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("crates/googleapis-tonic/Cargo.toml")?;
    let document = toml_edit::DocumentMut::from_str(&content)?;
    let features = document["features"]
        .as_table()
        .context("features table not found")?;
    for feature_name_as_str in features
        .iter()
        .map(|(feature_name_as_str, _)| feature_name_as_str)
        .filter(|it| !["default", "hash-map", "vec-u8", "btree-map", "bytes"].contains(it))
    {
        let command = format!("cargo check --features {}", feature_name_as_str);
        println!("{}", command);
        let output = std::process::Command::new("cargo")
            .args(["check", "--features", feature_name_as_str])
            .output()?;
        if !output.status.success() {
            std::io::Write::write_all(&mut std::io::stderr(), &output.stderr)?;
            anyhow::bail!("`{}` failed", command);
        }
    }
    Ok(())
}

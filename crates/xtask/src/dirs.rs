use std::env;
use std::path::PathBuf;

pub fn googleapis_dir() -> anyhow::Result<PathBuf> {
    let repository_root_dir = repository_root_dir()?;
    Ok(repository_root_dir
        .join("crates")
        .join("xtask")
        .join("googleapis"))
}

pub fn repository_root_dir() -> anyhow::Result<PathBuf> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(manifest_dir)
        .join("..")
        .join("..")
        .canonicalize()?)
}

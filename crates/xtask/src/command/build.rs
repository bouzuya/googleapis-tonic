mod build_crate;
mod build_crates;

use std::{fs, path::PathBuf};

use crate::proto_dir::ProtoDir;

#[derive(serde::Deserialize, serde::Serialize)]
struct StateFileContent {
    next_version: String,
}

/// Build `googleapis-tonic` and `googleapis-tonic-*` crates, and update the state file.
///
/// Output File Structure:
///
/// ```text
/// crates/
///   googleapis-tonic/
///     src/
///       bytes_btree_map/
///         FOO.BAR.BAZ.rs
///       bytes_hash_map/
///         ...
///       vec_u8_btree_map/
///         ...
///       vec_u8_hash_map/
///         ...
///       bytes_btree_map.rs
///       bytes_hash_map.rs
///       lib.rs
///       vec_u8_btree_map.rs
///       vec_u8_hash_map.rs
///     Cargo.toml
///   googleapis-tonic-FOO/
///     ...
///   xtask/
///     googleapis/            ... Input: googleapis/google repository (.proto files)
///     state.json             ... Input/Output: state file (next_version, publish_order)
///     ...
/// ```
pub fn execute() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("crates");
    let xtask_dir = crates_dir.join("xtask");
    let proto_dir = xtask_dir.join("googleapis");
    let proto_dir = ProtoDir::load(proto_dir)?;

    let state_file = xtask_dir.join("state.json");
    let state = fs::read_to_string(&state_file)?;
    let StateFileContent { next_version } = serde_json::from_str::<StateFileContent>(&state)?;
    let version = semver::Version::parse(&next_version)?;
    let next_version = semver::Version::new(version.major, version.minor + 1, version.patch);
    let state = serde_json::to_string_pretty(&StateFileContent {
        next_version: next_version.to_string(),
    })?;
    fs::write(&state_file, state)?;
    let version = version.to_string();

    let googleapis_tonic_src_dir = build_crate::build_crate(&crates_dir, &proto_dir, &version)?;
    build_crates::build_crates(&googleapis_tonic_src_dir, &proto_dir, &version)?;

    Ok(())
}

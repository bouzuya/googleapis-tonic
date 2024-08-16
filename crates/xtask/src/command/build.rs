mod build_crate;
mod build_crates;
mod state;

use std::fs;
use std::path::PathBuf;
use std::str::FromStr as _;

use self::state::State;
use crate::crate_name::CrateName;
use crate::proto_dir::ProtoDir;

/// Build `googleapis-tonic` and `googleapis-tonic-*` crates, and update the state file.
///
/// Output File Structure:
///
/// ```text
/// crates/
///   xtask/
///     googleapis/            ... Input: googleapis/google repository (.proto files)
///     state.json             ... Input/Output: state file (next_version, publish_order)
///     ...
/// generated/
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
/// ```
pub fn execute() -> anyhow::Result<()> {
    let generated_dir = PathBuf::from("generated");
    let xtask_dir = PathBuf::from("crates").join("xtask");

    // load googleapis/
    let proto_dir = xtask_dir.join("googleapis");
    let proto_dir = ProtoDir::load(proto_dir)?;

    // load state.json
    let state_file = xtask_dir.join("state.json");
    let state = State::load(&state_file)?;

    // remove generated/
    fs::remove_dir_all(&generated_dir)?;

    // generate generated/
    let crate_versions = state.crate_versions();
    let crate_version = crate_versions
        .get(&CrateName::from_str("googleapis-tonic")?)
        .cloned()
        .unwrap_or_default();
    let new_crate_version = build_crate::build_crate(&generated_dir, &proto_dir, &crate_version)?;
    let mut new_crate_versions =
        build_crates::build_crates(&generated_dir, &proto_dir, crate_versions)?;
    new_crate_versions.insert(CrateName::from_str("googleapis-tonic")?, new_crate_version);

    // update state.json
    let updated = state.update(&proto_dir, new_crate_versions)?;
    State::save(&state_file, &updated)?;
    Ok(())
}

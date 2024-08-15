mod build_crate;
mod build_crates;
mod state;

use std::path::PathBuf;

use self::state::State;
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
    let proto_dir = xtask_dir.join("googleapis");
    let proto_dir = ProtoDir::load(proto_dir)?;

    let state_file = xtask_dir.join("state.json");
    let state = State::load(&state_file)?;

    let version = state.current_version();
    build_crate::build_crate(&generated_dir, &proto_dir, &version)?;
    build_crates::build_crates(&generated_dir, &proto_dir, &version)?;

    let updated = state.update(&proto_dir)?;
    State::save(&state_file, &updated)?;
    Ok(())
}

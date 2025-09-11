mod build_crate;
mod build_crates;

use std::fs;
use std::str::FromStr as _;

use crate::crate_name::CrateName;
use crate::dirs;
use crate::googleapis::Googleapis;
use crate::state::State;

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
pub fn execute(force_update: bool) -> anyhow::Result<()> {
    let prost_version = "0.14.1";
    let tonic_version = "0.14.2";

    let generated_dir = dirs::generated_dir()?;
    let googleapis_dir = dirs::googleapis_dir()?;
    let xtask_dir = dirs::xtask_crate_dir()?;

    // load googleapis/
    println!("load googleapis/");
    let googleapis = Googleapis::load(googleapis_dir)?;

    // load state.json
    println!("load state.json");
    let state_file = xtask_dir.join("state.json");
    let state = State::load(&state_file)?;

    // remove generated/
    println!("remove generated/");
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir)?;
    }

    // generate generated/
    println!("generate generated/");
    let crate_versions = state.crate_versions();
    let crate_version = crate_versions
        .get(&CrateName::from_str("googleapis-tonic")?)
        .cloned()
        .unwrap_or_default();
    println!("  build crate");
    let new_crate_version = build_crate::build_crate(
        &generated_dir,
        &googleapis,
        &crate_version,
        state.package_hashes(),
        force_update,
        prost_version,
        tonic_version,
    )?;
    println!("  build crates");
    let mut new_crate_versions = build_crates::build_crates(
        &generated_dir,
        &googleapis,
        crate_versions,
        state.package_hashes(),
        force_update,
        prost_version,
        tonic_version,
    )?;
    new_crate_versions.insert(CrateName::from_str("googleapis-tonic")?, new_crate_version);

    // update state.json
    println!("update state.json");
    let updated = state.update(&googleapis, new_crate_versions)?;
    State::save(&state_file, &updated)?;
    Ok(())
}

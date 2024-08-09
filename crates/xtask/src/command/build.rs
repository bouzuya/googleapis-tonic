mod build_crate;
mod build_crates;

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fs,
    path::PathBuf,
};

use build_crates::CrateName;

use crate::{proto_dir::ProtoDir, protobuf_package_name::ProtobufPackageName};

#[derive(serde::Deserialize, serde::Serialize)]
struct StateFileContent {
    next_version: String,
    publish_order: Vec<String>,
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
    let StateFileContent {
        next_version,
        publish_order: _,
    } = serde_json::from_str::<StateFileContent>(&state)?;
    let version = semver::Version::parse(&next_version)?;
    let next_version = semver::Version::new(version.major, version.minor + 1, version.patch);
    let state = serde_json::to_string_pretty(&StateFileContent {
        next_version: next_version.to_string(),
        publish_order: {
            // topological sort
            let nodes = proto_dir.emit_package_names().to_owned();
            let mut edges = BTreeMap::new();
            let mut incoming_edge_counts = HashMap::<ProtobufPackageName, usize>::new();
            for (package_name, deps) in proto_dir.dependencies() {
                if !nodes.contains(package_name) {
                    continue;
                }
                let entry = incoming_edge_counts
                    .entry(package_name.to_owned())
                    .or_default();
                for dep in deps.iter().filter(|it| nodes.contains(it)) {
                    *entry += 1;
                    edges
                        .entry(dep.to_owned())
                        .or_insert_with(Vec::new)
                        .push(package_name.to_owned());
                }
            }
            let mut sorted = vec![];
            let mut no_incoming_edges = incoming_edge_counts
                .iter()
                .filter(|(_, count)| **count == 0)
                .map(|(node, _)| node.to_owned())
                .collect::<VecDeque<ProtobufPackageName>>();
            while let Some(node) = no_incoming_edges.pop_front() {
                for to in edges.get(&node).cloned().unwrap_or_default() {
                    if let Some(count) = incoming_edge_counts.get_mut(&to) {
                        *count -= 1;
                        if *count == 0 {
                            no_incoming_edges.push_back(to);
                        }
                    }
                }
                sorted.push(node);
            }
            sorted
                .iter()
                .map(CrateName::from_package_name)
                .map(|it| it.to_string())
                .collect::<Vec<String>>()
        },
    })?;
    fs::write(&state_file, state)?;
    let version = version.to_string();

    let googleapis_tonic_src_dir = build_crate::build_crate(&crates_dir, &proto_dir, &version)?;
    build_crates::build_crates(&googleapis_tonic_src_dir, &proto_dir, &version)?;

    Ok(())
}

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fs,
    path::Path,
};

use crate::{
    crate_name::CrateName, proto_dir::ProtoDir, protobuf_package_name::ProtobufPackageName,
};

#[derive(serde::Deserialize, serde::Serialize)]
struct StateFileContent {
    crate_versions: BTreeMap<String, String>,
    googleapis_version: String,
    next_version: String,
    publish_order: Vec<String>,
}

pub struct State(StateFileContent);

impl State {
    // TODO: remove I/O
    pub fn load(state_file: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(&state_file)?;
        Ok(Self(serde_json::from_str::<StateFileContent>(&content)?))
    }

    // TODO: remove I/O
    pub fn save(state_file: &Path, state: &State) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(&state.0)?;
        fs::write(state_file, &content)?;
        Ok(())
    }

    pub fn current_version(&self) -> String {
        self.0.next_version.clone()
    }

    pub fn update(&self, proto_dir: &ProtoDir) -> anyhow::Result<Self> {
        let next_version = &self.0.next_version;
        let state = StateFileContent {
            crate_versions: self
                .0
                .crate_versions
                .keys()
                .into_iter()
                .map(|key| (key.to_owned(), next_version.to_string()))
                .collect::<BTreeMap<String, String>>(),
            googleapis_version: proto_dir.version().to_string(),
            next_version: Self::next_version(&self.0.next_version)?,
            publish_order: Self::publish_order(proto_dir),
        };
        Ok(Self(state))
    }

    fn next_version(version: &str) -> anyhow::Result<String> {
        let version = semver::Version::parse(version)?;
        let next_version =
            semver::Version::new(version.major, version.minor + 1, version.patch).to_string();
        Ok(next_version)
    }

    fn publish_order(proto_dir: &ProtoDir) -> Vec<String> {
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
    }
}

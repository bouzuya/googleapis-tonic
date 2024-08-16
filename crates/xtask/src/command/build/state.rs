use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fs,
    path::Path,
    str::FromStr,
};

use crate::{
    crate_name::CrateName, crate_version::CrateVersion, googleapis_version::GoogleapisVersion,
    proto_dir::ProtoDir, protobuf_package_name::ProtobufPackageName,
};

#[derive(serde::Deserialize, serde::Serialize)]
struct StateFileContent {
    crate_versions: BTreeMap<String, String>,
    googleapis_version: String,
    next_version: String,
    publish_order: Vec<String>,
}

impl From<&State> for StateFileContent {
    fn from(
        State {
            crate_versions,
            googleapis_version,
            next_version,
            publish_order,
        }: &State,
    ) -> Self {
        Self {
            crate_versions: crate_versions
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<String, String>>(),
            googleapis_version: googleapis_version.to_string(),
            next_version: next_version.to_string(),
            publish_order: publish_order
                .into_iter()
                .map(|it| it.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

impl TryFrom<&StateFileContent> for State {
    type Error = anyhow::Error;

    fn try_from(
        StateFileContent {
            crate_versions,
            googleapis_version,
            next_version,
            publish_order,
        }: &StateFileContent,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            crate_versions: crate_versions
                .into_iter()
                .map(|(k, v)| Ok((CrateName::from_str(k)?, CrateVersion::from_str(v)?)))
                .collect::<anyhow::Result<BTreeMap<CrateName, CrateVersion>>>()?,
            googleapis_version: GoogleapisVersion::from_str(&googleapis_version)?,
            next_version: CrateVersion::from_str(&next_version)?,
            publish_order: publish_order
                .into_iter()
                .map(|it| CrateName::from_str(it))
                .collect::<anyhow::Result<Vec<CrateName>>>()?,
        })
    }
}

pub struct State {
    crate_versions: BTreeMap<CrateName, CrateVersion>,
    googleapis_version: GoogleapisVersion,
    next_version: CrateVersion,
    publish_order: Vec<CrateName>,
}

impl State {
    // TODO: remove I/O
    pub fn load(state_file: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(&state_file)?;
        let content = serde_json::from_str::<StateFileContent>(&content)?;
        State::try_from(&content)
    }

    // TODO: remove I/O
    pub fn save(state_file: &Path, state: &State) -> anyhow::Result<()> {
        let content = StateFileContent::from(state);
        let content = serde_json::to_string_pretty(&content)?;
        fs::write(state_file, &content)?;
        Ok(())
    }

    pub fn crate_versions(&self) -> &BTreeMap<CrateName, CrateVersion> {
        &self.crate_versions
    }

    pub fn update(
        &self,
        proto_dir: &ProtoDir,
        crate_versions: BTreeMap<CrateName, CrateVersion>,
    ) -> anyhow::Result<Self> {
        let googleapis_version = proto_dir.version().to_owned();
        let next_version = self.next_version.increment_minor();
        let publish_order = Self::publish_order(proto_dir);
        Ok(Self {
            crate_versions,
            googleapis_version,
            next_version,
            publish_order,
        })
    }

    fn publish_order(proto_dir: &ProtoDir) -> Vec<CrateName> {
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
            .collect::<Vec<CrateName>>()
    }
}

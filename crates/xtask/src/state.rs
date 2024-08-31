use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap},
    fs,
    path::Path,
    str::FromStr,
};

use crate::{
    crate_name::CrateName, crate_version::CrateVersion, googleapis::Googleapis,
    googleapis_version::GoogleapisVersion, protobuf_package_name::ProtobufPackageName,
    sha1hash::Sha1Hash,
};

pub struct State {
    crate_versions: BTreeMap<CrateName, CrateVersion>,
    googleapis_version: GoogleapisVersion,
    package_hashes: BTreeMap<ProtobufPackageName, Sha1Hash>,
    publish_order: Vec<CrateName>,
}

impl State {
    // TODO: remove I/O
    pub fn load(state_file: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(state_file)?;
        State::from_str(&content)
    }

    // TODO: remove I/O
    pub fn save(state_file: &Path, state: &State) -> anyhow::Result<()> {
        fs::write(state_file, state.to_string())?;
        Ok(())
    }

    pub fn crate_versions(&self) -> &BTreeMap<CrateName, CrateVersion> {
        &self.crate_versions
    }

    pub fn package_hashes(&self) -> &BTreeMap<ProtobufPackageName, Sha1Hash> {
        &self.package_hashes
    }

    pub fn publish_order(&self) -> &[CrateName] {
        &self.publish_order
    }

    pub fn update(
        &self,
        googleapis: &Googleapis,
        crate_versions: BTreeMap<CrateName, CrateVersion>,
    ) -> anyhow::Result<Self> {
        let googleapis_version = googleapis.version().to_owned();
        let package_hashes = googleapis.package_hashes().to_owned();
        let publish_order = Self::build_publish_order(
            googleapis.emit_package_names(),
            googleapis.package_dependencies(),
        );
        Ok(Self {
            crate_versions,
            googleapis_version,
            package_hashes,
            publish_order,
        })
    }

    fn build_publish_order(
        packages: &BTreeSet<ProtobufPackageName>,
        package_deps: &BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>,
    ) -> Vec<CrateName> {
        // topological sort
        let nodes = packages.to_owned();
        let (edges, mut incoming_edge_counts) = {
            let mut edges = BTreeMap::new();
            let mut incoming_edge_counts = packages
                .iter()
                .map(|it| (it.to_owned(), 0))
                .collect::<HashMap<ProtobufPackageName, usize>>();
            for (package_name, deps) in package_deps.iter().filter(|(it, _)| nodes.contains(it)) {
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
            (edges, incoming_edge_counts)
        };
        let mut sorted = vec![];
        let mut no_incoming_edges = incoming_edge_counts
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(node, _)| Reverse(node.to_owned()))
            .collect::<BinaryHeap<Reverse<ProtobufPackageName>>>();
        while let Some(Reverse(node)) = no_incoming_edges.pop() {
            for to in edges.get(&node).cloned().unwrap_or_default() {
                if let Some(count) = incoming_edge_counts.get_mut(&to) {
                    *count -= 1;
                    if *count == 0 {
                        no_incoming_edges.push(Reverse(to));
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

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = StateFileContent::from(self);
        let content = serde_json::to_string_pretty(&content).expect("serde_json::to_string_pretty");
        content.fmt(f)
    }
}

impl std::str::FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = serde_json::from_str::<StateFileContent>(s)?;
        State::try_from(&content)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct StateFileContent {
    crate_versions: BTreeMap<String, String>,
    googleapis_version: String,
    package_hashes: BTreeMap<String, String>,
    publish_order: Vec<String>,
}

impl From<&State> for StateFileContent {
    fn from(
        State {
            crate_versions,
            googleapis_version,
            package_hashes,
            publish_order,
        }: &State,
    ) -> Self {
        Self {
            crate_versions: crate_versions
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<String, String>>(),
            googleapis_version: googleapis_version.to_string(),
            package_hashes: package_hashes
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<String, String>>(),
            publish_order: publish_order
                .iter()
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
            package_hashes,
            publish_order,
        }: &StateFileContent,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            crate_versions: crate_versions
                .iter()
                .map(|(k, v)| Ok((CrateName::from_str(k)?, CrateVersion::from_str(v)?)))
                .collect::<anyhow::Result<BTreeMap<CrateName, CrateVersion>>>()?,
            googleapis_version: GoogleapisVersion::from_str(googleapis_version)?,
            package_hashes: package_hashes
                .iter()
                .map(|(k, v)| Ok((ProtobufPackageName::from_str(k)?, Sha1Hash::from_str(v)?)))
                .collect::<anyhow::Result<BTreeMap<ProtobufPackageName, Sha1Hash>>>()?,
            publish_order: publish_order
                .iter()
                .map(|it| CrateName::from_str(it))
                .collect::<anyhow::Result<Vec<CrateName>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_order_basic() -> anyhow::Result<()> {
        let packages = ["foo", "bar", "baz"]
            .into_iter()
            .map(ProtobufPackageName::from_str)
            .collect::<anyhow::Result<BTreeSet<ProtobufPackageName>>>()?;
        let package_deps = [("foo", vec!["bar"]), ("bar", vec!["baz"])]
            .into_iter()
            .map(|(k, v)| Ok((ProtobufPackageName::from_str(k)?, v.into_iter().map(ProtobufPackageName::from_str).collect::<anyhow::Result<BTreeSet<ProtobufPackageName>>>()?)))
            .collect::<anyhow::Result<BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>>>()?;
        assert_eq!(
            State::build_publish_order(&packages, &package_deps),
            vec![
                CrateName::from_str("googleapis-tonic-baz")?,
                CrateName::from_str("googleapis-tonic-bar")?,
                CrateName::from_str("googleapis-tonic-foo")?,
            ]
        );
        Ok(())
    }

    #[test]
    fn test_publish_order_stable1() -> anyhow::Result<()> {
        let packages = ["a", "b1", "b2", "c"]
            .into_iter()
            .map(ProtobufPackageName::from_str)
            .collect::<anyhow::Result<BTreeSet<ProtobufPackageName>>>()?;
        let package_deps = [
            ("a", vec!["b1", "b2"]),
            ("b1", vec!["c"]),
            ("b2", vec!["c"]),
        ]
        .into_iter()
        .map(|(k, v)| {
            Ok((
                ProtobufPackageName::from_str(k)?,
                v.into_iter()
                    .map(ProtobufPackageName::from_str)
                    .collect::<anyhow::Result<BTreeSet<ProtobufPackageName>>>()?,
            ))
        })
        .collect::<anyhow::Result<BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>>>(
        )?;
        assert_eq!(
            State::build_publish_order(&packages, &package_deps),
            vec![
                CrateName::from_str("googleapis-tonic-c")?,
                CrateName::from_str("googleapis-tonic-b1")?,
                CrateName::from_str("googleapis-tonic-b2")?,
                CrateName::from_str("googleapis-tonic-a")?,
            ]
        );
        Ok(())
    }

    #[test]
    fn test_publish_order_stable2() -> anyhow::Result<()> {
        let packages = ["a", "b1", "b2", "c", "d"]
            .into_iter()
            .map(ProtobufPackageName::from_str)
            .collect::<anyhow::Result<BTreeSet<ProtobufPackageName>>>()?;
        let package_deps = [
            ("a", vec!["b1", "b2"]),
            ("d", vec!["c"]),
            ("b1", vec!["b2", "c"]),
            ("b2", vec!["c"]),
        ]
        .into_iter()
        .map(|(k, v)| {
            Ok((
                ProtobufPackageName::from_str(k)?,
                v.into_iter()
                    .map(ProtobufPackageName::from_str)
                    .collect::<anyhow::Result<BTreeSet<ProtobufPackageName>>>()?,
            ))
        })
        .collect::<anyhow::Result<BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>>>(
        )?;
        assert_eq!(
            State::build_publish_order(&packages, &package_deps),
            vec![
                CrateName::from_str("googleapis-tonic-c")?,
                CrateName::from_str("googleapis-tonic-b2")?,
                CrateName::from_str("googleapis-tonic-b1")?,
                CrateName::from_str("googleapis-tonic-a")?,
                CrateName::from_str("googleapis-tonic-d")?,
            ]
        );
        Ok(())
    }
}

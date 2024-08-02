use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    str::FromStr as _,
};

use anyhow::Context as _;

use crate::{package_name::PackageName, proto_file::ProtoFile};

pub struct ProtoDir {
    dependencies: BTreeMap<PackageName, BTreeSet<PackageName>>,
    dir_path: PathBuf,
    proto_paths: Vec<PathBuf>,
}

impl ProtoDir {
    pub fn load<P: Into<PathBuf>>(proto_dir: P) -> anyhow::Result<Self> {
        let dir_path: PathBuf = proto_dir.into();

        let mut all_proto_files = BTreeMap::<PathBuf, ProtoFile>::new();
        let proto_paths = Self::proto_paths_from_dir(&dir_path)?;
        for path in &proto_paths {
            let content = fs::read_to_string(path)?;
            let proto_file = ProtoFile::from_str(&content)?;
            all_proto_files.insert(path.strip_prefix(&dir_path)?.to_owned(), proto_file);
        }

        let mut dependencies = BTreeMap::<PackageName, BTreeSet<PackageName>>::new();
        for proto_file in all_proto_files.values() {
            dependencies
                .entry(proto_file.package_name().to_owned())
                .or_default()
                .extend(Self::proto_file_dependencies(&all_proto_files, proto_file)?);
        }

        Ok(Self {
            dependencies,
            dir_path,
            proto_paths,
        })
    }

    pub fn dependencies(&self) -> &BTreeMap<PackageName, BTreeSet<PackageName>> {
        &self.dependencies
    }

    pub fn dir_path(&self) -> &Path {
        &self.dir_path
    }

    pub fn proto_paths(&self) -> &[PathBuf] {
        &self.proto_paths
    }

    fn proto_file_dependencies(
        all_proto_files: &BTreeMap<PathBuf, ProtoFile>,
        proto_file: &ProtoFile,
    ) -> anyhow::Result<BTreeSet<PackageName>> {
        proto_file
            .import_paths()
            .iter()
            .filter(|it| !it.starts_with("google/protobuf"))
            .map(|it| {
                all_proto_files.get(it).with_context(|| {
                    format!(
                        "proto_file not found: import_path = {:?} (by {:?})",
                        it, proto_file
                    )
                })
            })
            .try_fold(
                {
                    let mut set = BTreeSet::<PackageName>::new();
                    set.insert(proto_file.package_name().to_owned());
                    set
                },
                |mut acc, it| -> anyhow::Result<BTreeSet<PackageName>> {
                    acc.extend(Self::proto_file_dependencies(all_proto_files, it?)?);
                    Ok(acc)
                },
            )
    }

    fn proto_paths_from_dir<P: AsRef<Path>>(dir: P) -> anyhow::Result<Vec<PathBuf>> {
        let dir: &Path = dir.as_ref();
        let mut paths = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "proto" {
                        paths.push(path);
                    }
                }
            } else {
                let path_buf = entry.path();
                paths.append(&mut Self::proto_paths_from_dir(&path_buf)?);
            }
        }
        Ok(paths)
    }
}

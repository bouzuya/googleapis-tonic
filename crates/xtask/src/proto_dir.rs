use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    str::FromStr as _,
};

use anyhow::Context as _;

use crate::{proto_file::ProtoFile, protobuf_package_name::ProtobufPackageName};

pub struct ProtoDir {
    dependencies: BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>,
    dir_path: PathBuf,
    emit_package_names: BTreeSet<ProtobufPackageName>,
    proto_paths: Vec<PathBuf>,
}

impl ProtoDir {
    pub fn load<P: Into<PathBuf>>(proto_dir: P) -> anyhow::Result<Self> {
        let dir_path: PathBuf = proto_dir.into();

        let mut all_proto_files = BTreeMap::<PathBuf, ProtoFile>::new();
        let proto_file_paths = Self::proto_file_paths_from_dir(&dir_path)?;
        for path in &proto_file_paths {
            let content = fs::read_to_string(path)?;
            let proto_file = ProtoFile::from_str(&content)?;
            all_proto_files.insert(path.strip_prefix(&dir_path)?.to_owned(), proto_file);
        }

        let mut emit_package_name_candidates = BTreeSet::<ProtobufPackageName>::new();
        for proto_dir_path in Self::proto_dir_paths_from_dir(&dir_path)? {
            let package_name = proto_dir_path
                .strip_prefix(&dir_path)?
                .to_owned()
                .to_str()
                .context("proto_dir_path is not valid UTF-8")?
                .split(std::path::MAIN_SEPARATOR)
                .collect::<Vec<&str>>()
                .join(".");
            match ProtobufPackageName::from_str(&package_name) {
                Err(_) => {
                    // skip invalid package name
                }
                Ok(package_name) => {
                    emit_package_name_candidates.insert(package_name);
                }
            }
        }

        let mut dependencies =
            BTreeMap::<ProtobufPackageName, BTreeSet<ProtobufPackageName>>::new();
        for proto_file in all_proto_files.values() {
            dependencies
                .entry(proto_file.package_name().to_owned())
                .or_default()
                .extend(Self::proto_file_dependencies(&all_proto_files, proto_file)?);
        }
        for (package_name, deps) in dependencies.iter_mut() {
            deps.remove(package_name);
        }
        let mut emit_package_names = BTreeSet::<ProtobufPackageName>::new();
        for package_name in dependencies.keys() {
            if emit_package_name_candidates.contains(package_name) {
                emit_package_names.insert(package_name.to_owned());
            }
        }

        Ok(Self {
            dependencies,
            dir_path,
            emit_package_names,
            proto_paths: proto_file_paths,
        })
    }

    pub fn dependencies(&self) -> &BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>> {
        &self.dependencies
    }

    pub fn emit_package_names(&self) -> &BTreeSet<ProtobufPackageName> {
        &self.emit_package_names
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
    ) -> anyhow::Result<BTreeSet<ProtobufPackageName>> {
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
                    let mut set = BTreeSet::<ProtobufPackageName>::new();
                    set.insert(proto_file.package_name().to_owned());
                    set
                },
                |mut acc, it| -> anyhow::Result<BTreeSet<ProtobufPackageName>> {
                    acc.extend(Self::proto_file_dependencies(all_proto_files, it?)?);
                    Ok(acc)
                },
            )
    }

    fn proto_dir_paths_from_dir<P: AsRef<Path>>(dir: P) -> anyhow::Result<Vec<PathBuf>> {
        let dir: &Path = dir.as_ref();
        let mut paths = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                paths.push(entry.path());
                paths.append(&mut Self::proto_dir_paths_from_dir(entry.path())?);
            }
        }
        Ok(paths)
    }
    fn proto_file_paths_from_dir<P: AsRef<Path>>(dir: P) -> anyhow::Result<Vec<PathBuf>> {
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
                paths.append(&mut Self::proto_file_paths_from_dir(&path_buf)?);
            }
        }
        Ok(paths)
    }
}

use crate::{googleapis_version::GoogleapisVersion, proto_file_path::ProtoFilePath};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
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
    proto_file_paths: Vec<ProtoFilePath>,
    version: GoogleapisVersion,
}

impl ProtoDir {
    pub fn load<P: Into<PathBuf>>(proto_dir: P) -> anyhow::Result<Self> {
        let dir_path: PathBuf = proto_dir.into();
        let dir_path = dir_path.canonicalize()?;

        let version = GoogleapisVersion::load_from_googleapis_dir(&dir_path)?;
        let proto_file_paths = Self::proto_file_paths_from_dir(&dir_path, &dir_path)?;
        let proto_dir_paths = Self::proto_dir_paths_from_dir(&dir_path)?;

        let mut all_proto_files = BTreeMap::<ProtoFilePath, ProtoFile>::new();
        for proto_file_path in &proto_file_paths {
            let content = fs::read_to_string(dir_path.join(proto_file_path.to_path_buf()))?;
            let proto_file = ProtoFile::from_str(&content)?;
            all_proto_files.insert(proto_file_path.to_owned(), proto_file);
        }

        let mut emit_package_name_candidates = BTreeSet::<ProtobufPackageName>::new();
        for proto_dir_path in proto_dir_paths {
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

        // max package name length is 47. crates.io max crate name length is 64. googleapis-tonic- length is 17.
        let mut invalid_package_names = dependencies
            .keys()
            .filter(|it| it.to_string().len() > 47)
            .cloned()
            .collect::<VecDeque<ProtobufPackageName>>();
        while let Some(invalid_package_name) = invalid_package_names.pop_front() {
            if emit_package_names.remove(&invalid_package_name) {
                for (pkg, deps) in dependencies.iter() {
                    if deps.contains(&invalid_package_name) {
                        invalid_package_names.push_back(pkg.to_owned());
                    }
                }
            }
        }

        Ok(Self {
            dependencies,
            dir_path,
            emit_package_names,
            proto_file_paths,
            version,
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

    pub fn proto_paths(&self) -> Vec<PathBuf> {
        self.proto_file_paths
            .iter()
            .map(|it| self.dir_path.join(it.to_path_buf()))
            .collect::<Vec<PathBuf>>()
    }

    pub fn version(&self) -> &GoogleapisVersion {
        &self.version
    }

    fn proto_file_dependencies(
        all_proto_files: &BTreeMap<ProtoFilePath, ProtoFile>,
        proto_file: &ProtoFile,
    ) -> anyhow::Result<BTreeSet<ProtobufPackageName>> {
        proto_file
            .import_paths()
            .iter()
            .filter(|it| !it.is_google_protobuf())
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

    fn proto_file_paths_from_dir<P: AsRef<Path>>(
        dir: P,
        proto_dir: &Path,
    ) -> anyhow::Result<Vec<ProtoFilePath>> {
        let dir: &Path = dir.as_ref();
        let mut paths = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                let file_path = entry.path();
                if let Some(extension) = file_path.extension() {
                    if extension == "proto" {
                        let p = ProtoFilePath::from_absolute_path(&file_path, proto_dir)?;
                        paths.push(p);
                    }
                }
            } else if file_type.is_dir() {
                let dir_path = entry.path();
                paths.append(&mut Self::proto_file_paths_from_dir(&dir_path, proto_dir)?);
            } else {
                // do nothing
            }
        }
        Ok(paths)
    }
}

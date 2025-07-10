use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
    str::FromStr as _,
};

use anyhow::Context as _;

use crate::googleapis_version::GoogleapisVersion;
use crate::proto_file::ProtoFile;
use crate::proto_file_path::ProtoFilePath;
use crate::protobuf_package_name::ProtobufPackageName;
use crate::sha1hash::Sha1Hash;

pub struct Googleapis {
    dir_path: PathBuf,
    emit_package_names: BTreeSet<ProtobufPackageName>,
    package_deps: BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>,
    package_hashes: BTreeMap<ProtobufPackageName, Sha1Hash>,
    proto_file_paths: BTreeSet<ProtoFilePath>,
    version: GoogleapisVersion,
}

impl Googleapis {
    pub fn load<P: Into<PathBuf>>(googleapis_dir: P) -> anyhow::Result<Self> {
        let dir_path: PathBuf = googleapis_dir.into();
        let dir_path = dir_path.canonicalize()?;

        let version = GoogleapisVersion::load_from_googleapis_dir(&dir_path)?;
        let proto_file_paths = Self::proto_file_paths_from_dir(&dir_path, &dir_path)?;

        let (package_deps, package_hashes) = Self::load_packages(&dir_path, &proto_file_paths)?;

        let emit_package_names = {
            let emit_package_name_candidates = {
                let mut set = BTreeSet::<ProtobufPackageName>::new();
                let proto_dir_paths = Self::proto_dir_paths_from_dir(&dir_path)?;
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
                            set.insert(package_name);
                        }
                    }
                }
                set
            };

            let mut emit_package_names = BTreeSet::<ProtobufPackageName>::new();
            for package_name in package_deps.keys() {
                if emit_package_name_candidates.contains(package_name) {
                    emit_package_names.insert(package_name.to_owned());
                }
            }

            const CRATES_IO_CRATE_NAME_MAX_LENGTH: usize = 64;
            const CRATE_NAME_PREFIX_LENGTH: usize = "googleapis-tonic-".len();
            let mut invalid_package_names = package_deps
                .keys()
                .filter(|it| {
                    // TODO: skip google.cloud.storageinsights.v1
                    // <https://github.com/bouzuya/googleapis-tonic/actions/runs/16189950077/job/45703200056>
                    it.to_string() == "google.cloud.storageinsights.v1"
                        || CRATE_NAME_PREFIX_LENGTH + it.to_string().len()
                            > CRATES_IO_CRATE_NAME_MAX_LENGTH
                })
                .cloned()
                .collect::<VecDeque<ProtobufPackageName>>();
            while let Some(invalid_package_name) = invalid_package_names.pop_front() {
                if emit_package_names.remove(&invalid_package_name) {
                    for (pkg, deps) in package_deps.iter() {
                        if deps.contains(&invalid_package_name) {
                            invalid_package_names.push_back(pkg.to_owned());
                        }
                    }
                }
            }
            emit_package_names
        };

        Ok(Self {
            dir_path,
            emit_package_names,
            package_deps,
            package_hashes,
            proto_file_paths,
            version,
        })
    }

    pub fn dir_path(&self) -> &Path {
        &self.dir_path
    }

    pub fn emit_package_names(&self) -> &BTreeSet<ProtobufPackageName> {
        &self.emit_package_names
    }

    pub fn package_dependencies(
        &self,
    ) -> &BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>> {
        &self.package_deps
    }

    pub fn package_hashes(&self) -> &BTreeMap<ProtobufPackageName, Sha1Hash> {
        &self.package_hashes
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

    #[allow(clippy::type_complexity)]
    fn load_packages(
        dir_path: &Path,
        proto_file_paths: &BTreeSet<ProtoFilePath>,
    ) -> anyhow::Result<(
        BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>,
        BTreeMap<ProtobufPackageName, Sha1Hash>,
    )> {
        let all_proto_files = {
            let mut map = BTreeMap::<ProtoFilePath, ProtoFile>::new();
            for proto_file_path in proto_file_paths {
                let content = fs::read_to_string(dir_path.join(proto_file_path.to_path_buf()))?;
                let proto_file = ProtoFile::from_str(&content)?;
                map.insert(proto_file_path.to_owned(), proto_file);
            }
            map
        };

        let package_files = {
            let mut map = BTreeMap::<ProtobufPackageName, BTreeSet<ProtoFilePath>>::new();
            for (proto_file_path, proto_file) in all_proto_files.iter() {
                map.entry(proto_file.package_name().to_owned())
                    .or_default()
                    .insert(proto_file_path.to_owned());
            }
            map
        };

        let mut package_deps =
            BTreeMap::<ProtobufPackageName, BTreeSet<ProtobufPackageName>>::new();
        for proto_file in all_proto_files.values() {
            package_deps
                .entry(proto_file.package_name().to_owned())
                .or_default()
                .extend(Self::proto_file_dependencies(&all_proto_files, proto_file)?);
        }
        for (package_name, deps) in package_deps.iter_mut() {
            deps.remove(package_name);
        }

        let package_hashes = {
            let mut package_hashes = BTreeMap::<ProtobufPackageName, Sha1Hash>::new();
            for (package_name, package_file_paths) in package_files.iter() {
                let mut s = String::new();
                for package_file_path in package_file_paths {
                    let proto_file = all_proto_files.get(package_file_path).context("")?;
                    let proto_file_content_hash = proto_file.content_hash();
                    s.push_str(&proto_file_content_hash.to_string());
                }
                let package_hash = Sha1Hash::digest(&s);
                package_hashes.insert(package_name.to_owned(), package_hash);
            }
            package_hashes
        };
        Ok((package_deps, package_hashes))
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
    ) -> anyhow::Result<BTreeSet<ProtoFilePath>> {
        let dir: &Path = dir.as_ref();
        let mut paths = BTreeSet::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                let file_path = entry.path();
                if let Some(extension) = file_path.extension() {
                    if extension == "proto" {
                        let p = ProtoFilePath::from_absolute_path(&file_path, proto_dir)?;
                        paths.insert(p);
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

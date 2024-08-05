mod build_crates;

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    str::FromStr as _,
};

use anyhow::Context;

use crate::bytes_type::BytesType;
use crate::feature_name::FeatureName;
use crate::map_type::MapType;
use crate::modules::Modules;
use crate::proto_dir::ProtoDir;

pub fn execute() -> anyhow::Result<()> {
    let proto_dir = "crates/xtask/googleapis";
    let src_dir = "crates/googleapis-tonic/src";

    let proto_dir = ProtoDir::load(proto_dir)?;

    for (bytes_type, map_type) in BytesType::values().iter().flat_map(|bytes_type| {
        MapType::values()
            .iter()
            .map(|map_type| (*bytes_type, *map_type))
            .collect::<Vec<(BytesType, MapType)>>()
    }) {
        let root_mod_name = format!("{}_{}", bytes_type.as_path_part(), map_type.as_path_part());
        let out_dir = format!("{}/{}", src_dir, root_mod_name);

        let mut prost_config = prost_build::Config::new();
        let packages = proto_dir
            .dependencies()
            .keys()
            .map(ToString::to_string)
            .map(|it| format!(".{}", it))
            .collect::<Vec<String>>();
        prost_config.disable_comments(packages.clone());
        tonic_build::configure()
            .btree_map(match map_type {
                MapType::BTreeMap => vec!["."],
                MapType::HashMap => vec![],
            })
            .build_client(true)
            .build_server(false)
            .build_transport(false)
            .bytes(match bytes_type {
                BytesType::Bytes => vec!["."],
                BytesType::VecU8 => vec![],
            })
            .disable_comments(".")
            .emit_rerun_if_changed(false)
            .out_dir(out_dir.as_str())
            .protoc_arg("--experimental_allow_proto3_optional")
            .compile_with_config(
                prost_config,
                proto_dir.proto_paths(),
                &[proto_dir.dir_path()],
            )?;

        let mut file_names = vec![];
        for dir_entry in fs::read_dir(out_dir.as_str())? {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            let file_name = path
                .file_name()
                .with_context(|| format!("file_name is None {}", path.display()))?
                .to_str()
                .with_context(|| format!("file_name is not utf-8 {}", path.display()))?;
            file_names.push(file_name.to_owned());
        }

        let modules = Modules::from_file_names(&file_names);
        let output = modules.to_rs_file_content(&format!("{}/", root_mod_name));
        fs::write(format!("{}/{}.rs", src_dir, root_mod_name), output)?;
    }

    update_cargo_toml(src_dir, proto_dir)?;

    build_crates::build_crates(src_dir)?;

    Ok(())
}

fn build_features(proto_dir: ProtoDir) -> BTreeMap<FeatureName, BTreeSet<FeatureName>> {
    let mut features = BTreeMap::new();
    features.insert(
        FeatureName::default(),
        [
            FeatureName::from(MapType::HashMap),
            FeatureName::from(BytesType::VecU8),
        ]
        .into_iter()
        .collect::<BTreeSet<FeatureName>>(),
    );
    for bytes_type in BytesType::values() {
        features.insert(FeatureName::from(*bytes_type), BTreeSet::default());
    }
    for map_type in MapType::values() {
        features.insert(FeatureName::from(*map_type), BTreeSet::default());
    }
    for (pkg, deps) in proto_dir.dependencies() {
        let feature_name = FeatureName::from(pkg);
        let deps = deps
            .iter()
            .map(FeatureName::from)
            .filter(|it| it != &feature_name)
            .collect::<BTreeSet<FeatureName>>();
        features.insert(feature_name, deps);
    }
    features
}

fn update_cargo_toml(src_dir: &str, proto_dir: ProtoDir) -> anyhow::Result<()> {
    let cargo_toml_path = PathBuf::from(src_dir)
        .join("../Cargo.toml")
        .canonicalize()?;
    let cargo_toml = fs::read_to_string(&cargo_toml_path)?;
    let mut document = toml_edit::DocumentMut::from_str(&cargo_toml)?;
    let table = document["features"]
        .as_table_mut()
        .context("features is not a table")?;
    table.clear();
    for (feature_name, deps) in build_features(proto_dir) {
        table.insert(
            &feature_name.to_string(),
            toml_edit::Item::Value(toml_edit::Value::Array(
                deps.iter()
                    .map(ToString::to_string)
                    .collect::<toml_edit::Array>(),
            )),
        );
    }
    table.sort_values();
    fs::write(cargo_toml_path, document.to_string())?;
    Ok(())
}

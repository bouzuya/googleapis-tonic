mod bytes_type;
mod feature_name;
mod ident;
mod map_type;
mod module;
mod modules;
mod package_name;
mod proto_dir;
mod proto_file;

use std::{fs, path::PathBuf, str::FromStr as _};

use anyhow::Context;

use bytes_type::BytesType;
use map_type::MapType;
use modules::Modules;
use package_name::PackageName;
use proto_dir::ProtoDir;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Build,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Build => build(),
    }
}

fn build() -> anyhow::Result<()> {
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
            .out_dir(out_dir.as_str())
            .protoc_arg("--experimental_allow_proto3_optional")
            .compile(proto_dir.proto_paths(), &[proto_dir.dir_path()])?;

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

    let mut file_names = vec![];
    for dir_entry in fs::read_dir(format!("{}/vec_u8_hash_map", src_dir))? {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .with_context(|| format!("file_name is None {}", path.display()))?
            .to_str()
            .with_context(|| format!("file_name is not utf-8 {}", path.display()))?;
        file_names.push(file_name.to_owned());
    }

    let cargo_toml_path = PathBuf::from(src_dir)
        .join("../Cargo.toml")
        .canonicalize()?;
    let cargo_toml = fs::read_to_string(&cargo_toml_path)?;
    let mut document = toml_edit::DocumentMut::from_str(&cargo_toml)?;
    let table = document["features"]
        .as_table_mut()
        .context("features is not a table")?;
    table.clear();
    table.insert(
        "default",
        toml_edit::Item::from_str(r#"["hash-map", "vec-u8"]"#)?,
    );
    let value_of_empty_array =
        toml_edit::Item::Value(toml_edit::Value::Array(toml_edit::Array::default()));
    for bytes_type in BytesType::values() {
        table.insert(bytes_type.as_feature_name(), value_of_empty_array.clone());
    }
    for map_type in MapType::values() {
        table.insert(map_type.as_feature_name(), value_of_empty_array.clone());
    }
    for (pkg, deps) in proto_dir.dependencies() {
        fn pkg_to_feature_name(pkg: &PackageName) -> String {
            pkg.to_string().split('.').collect::<Vec<&str>>().join("-")
        }
        let feature_name = pkg_to_feature_name(pkg);
        table.insert(
            &feature_name,
            toml_edit::Item::Value(toml_edit::Value::Array({
                let mut array = toml_edit::Array::default();
                for dep in deps {
                    let f = pkg_to_feature_name(dep);
                    if f == feature_name {
                        continue;
                    }
                    array.push(f);
                }
                array
            })),
        );
    }
    table.sort_values();
    fs::write(cargo_toml_path, document.to_string())?;

    Ok(())
}

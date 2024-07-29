mod feature_name;
mod ident;
mod module;
mod modules;

use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr as _,
};

use anyhow::Context;

use modules::Modules;

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

    enum MapType {
        BTreeMap,
        HashMap,
    }

    for map_type in &[MapType::BTreeMap, MapType::HashMap] {
        let out_dir = match map_type {
            MapType::BTreeMap => format!("{}/btree_map", src_dir),
            MapType::HashMap => format!("{}/hash_map", src_dir),
        };

        let proto_paths = proto_paths_from_dir(proto_dir)?;
        tonic_build::configure()
            .btree_map(match map_type {
                MapType::BTreeMap => vec!["."],
                MapType::HashMap => vec![],
            })
            .build_client(true)
            // don't generate server code
            .build_server(false)
            .build_transport(false)
            // use bytes::Bytes instead of Vec<u8>
            .bytes(["."])
            .out_dir(out_dir.as_str())
            .protoc_arg("--experimental_allow_proto3_optional")
            .compile(&proto_paths, &[proto_dir])?;

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
        let output = modules.to_rs_file_content();
        fs::write(
            format!(
                "{}/{}.rs",
                src_dir,
                match map_type {
                    MapType::BTreeMap => "btree_map",
                    MapType::HashMap => "hash_map",
                }
            ),
            output,
        )?;
    }

    let mut file_names = vec![];
    for dir_entry in fs::read_dir(format!("{}/btree_map", src_dir))? {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .with_context(|| format!("file_name is None {}", path.display()))?
            .to_str()
            .with_context(|| format!("file_name is not utf-8 {}", path.display()))?;
        if file_name == "lib.rs" {
            continue;
        }
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
    table.insert("default", toml_edit::Item::from_str(r#"["hash-map"]"#)?);
    let value_of_empty_array =
        toml_edit::Item::Value(toml_edit::Value::Array(toml_edit::Array::default()));
    table.insert("hash-map", value_of_empty_array.clone());
    table.insert("btree-map", value_of_empty_array.clone());
    for file_name in file_names {
        table.insert(
            &file_name
                .split('.')
                .filter(|s| s != &"rs")
                .map(|s| s.replace("r#", ""))
                .collect::<Vec<String>>()
                .join("-"),
            value_of_empty_array.clone(),
        );
    }
    table.sort_values();
    fs::write(cargo_toml_path, document.to_string())?;

    Ok(())
}

fn proto_paths_from_dir<P: AsRef<Path>>(dir: P) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths = vec![];
    for entry in std::fs::read_dir(dir)? {
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
            paths.append(&mut proto_paths_from_dir(&path_buf)?);
        }
    }
    Ok(paths)
}

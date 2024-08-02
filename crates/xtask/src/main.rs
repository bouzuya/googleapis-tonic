mod feature_name;
mod ident;
mod module;
mod modules;
mod package_name;
mod proto_file;

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

    #[derive(Clone, Copy)]
    enum BytesType {
        Bytes,
        VecU8,
    }

    #[derive(Clone, Copy)]
    enum MapType {
        BTreeMap,
        HashMap,
    }

    for (bytes_type, map_type) in [BytesType::Bytes, BytesType::VecU8]
        .into_iter()
        .flat_map(|bytes_type| {
            [MapType::BTreeMap, MapType::HashMap]
                .into_iter()
                .map(|map_type| (bytes_type, map_type))
                .collect::<Vec<(BytesType, MapType)>>()
        })
        .collect::<Vec<(BytesType, MapType)>>()
    {
        let out_dir = format!(
            "{}/{}_{}",
            src_dir,
            match bytes_type {
                BytesType::Bytes => "bytes",
                BytesType::VecU8 => "vec_u8",
            },
            match map_type {
                MapType::BTreeMap => "btree_map",
                MapType::HashMap => "hash_map",
            }
        );

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
            .bytes(match bytes_type {
                BytesType::Bytes => vec!["."],
                BytesType::VecU8 => vec![],
            })
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
        let output = modules.to_rs_file_content(&format!(
            "{}_{}/",
            match bytes_type {
                BytesType::Bytes => "bytes",
                BytesType::VecU8 => "vec_u8",
            },
            match map_type {
                MapType::BTreeMap => "btree_map",
                MapType::HashMap => "hash_map",
            }
        ));
        fs::write(
            format!(
                "{}/{}_{}.rs",
                src_dir,
                match bytes_type {
                    BytesType::Bytes => "bytes",
                    BytesType::VecU8 => "vec_u8",
                },
                match map_type {
                    MapType::BTreeMap => "btree_map",
                    MapType::HashMap => "hash_map",
                }
            ),
            output,
        )?;
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
        // FIXME: Add "google-rpc" and "google-type" if needed
        toml_edit::Item::from_str(r#"["google-rpc", "google-type", "vec-u8", "hash-map"]"#)?,
    );
    let value_of_empty_array =
        toml_edit::Item::Value(toml_edit::Value::Array(toml_edit::Array::default()));
    table.insert("bytes", value_of_empty_array.clone());
    table.insert("vec-u8", value_of_empty_array.clone());
    table.insert("btree-map", value_of_empty_array.clone());
    table.insert("hash-map", value_of_empty_array.clone());
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

use std::path::{Path, PathBuf};

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
    let proto_paths = proto_paths_from_dir(proto_dir)?;
    tonic_build::configure()
        // use BTreeMap instead of HashMap
        .btree_map(["."])
        .build_client(true)
        // don't generate server code
        .build_server(false)
        .build_transport(true)
        // use bytes::Bytes instead of Vec<u8>
        .bytes(["."])
        .out_dir(
            // FIXME:
            "crates/xtask/tonic-generated",
        )
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&proto_paths, &[proto_dir])?;

    // FIXME: crates/xtask/tonic-generated/ -> crates/googleapis-tonic/src/

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

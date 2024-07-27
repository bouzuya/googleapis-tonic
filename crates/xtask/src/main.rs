use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

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

struct Mod {
    include: bool,
    mods: BTreeMap<String, Mod>,
}

fn mods_to_string(mods: &BTreeMap<String, Mod>) -> String {
    fn dfs(mods: &BTreeMap<String, Mod>, c: &mut Vec<String>, s: &mut String) {
        for (name, r#mod) in mods {
            s.push_str(&format!("{}pub mod {} {{\n", "  ".repeat(c.len()), name));
            c.push(name.clone());
            if r#mod.include {
                s.push_str(&format!(
                    "{}include!(\"{}.rs\");\n",
                    "  ".repeat(c.len()),
                    c.join("."),
                ));
            }
            dfs(&r#mod.mods, c, s);
            c.pop();
            s.push_str(&format!("{}}}\n", "  ".repeat(c.len())));
        }
    }

    let mut s = String::new();
    let mut c = vec![];
    dfs(mods, &mut c, &mut s);
    s
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{mods_to_string, Mod};

    #[test]
    fn test_mods_to_string() {
        let root = [(
            "google".to_owned(),
            Mod {
                include: false,
                mods: [(
                    "firestore".to_owned(),
                    Mod {
                        include: true,
                        mods: [
                            (
                                "v1".to_owned(),
                                Mod {
                                    include: true,
                                    mods: BTreeMap::new(),
                                },
                            ),
                            (
                                "v1beta1".to_owned(),
                                Mod {
                                    include: true,
                                    mods: BTreeMap::new(),
                                },
                            ),
                        ]
                        .into_iter()
                        .collect::<BTreeMap<String, Mod>>(),
                    },
                )]
                .into_iter()
                .collect::<BTreeMap<String, Mod>>(),
            },
        )]
        .into_iter()
        .collect::<BTreeMap<String, Mod>>();

        assert_eq!(
            mods_to_string(&root),
            r#"pub mod google {
  pub mod firestore {
    include!("google.firestore.rs");
    pub mod v1 {
      include!("google.firestore.v1.rs");
    }
    pub mod v1beta1 {
      include!("google.firestore.v1beta1.rs");
    }
  }
}
"#
        );
    }
}

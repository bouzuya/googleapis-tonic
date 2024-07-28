use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;

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
    let proto_paths = proto_paths_from_dir(proto_dir)?;
    tonic_build::configure()
        // use BTreeMap instead of HashMap
        .btree_map(["."])
        .build_client(true)
        // don't generate server code
        .build_server(false)
        .build_transport(false)
        // use bytes::Bytes instead of Vec<u8>
        .bytes(["."])
        .out_dir(src_dir)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&proto_paths, &[proto_dir])?;

    let mut file_names = vec![];
    for dir_entry in fs::read_dir(src_dir)? {
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

    let mods = mods_from_file_names(&file_names);
    let output = mods_to_string(&mods);
    fs::write(format!("{}/lib.rs", src_dir), output)?;

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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Mod {
    ident: String,
    include: bool,
    mods: Vec<Mod>,
}

fn mods_from_file_names(paths: &[String]) -> Vec<Mod> {
    fn get_or_push<'a>(mods: &'a mut Vec<Mod>, ident: &str) -> &'a mut Mod {
        match mods.iter().position(|m| m.ident == ident) {
            Some(index) => mods.get_mut(index).expect("mod is included"),
            None => {
                mods.push(Mod {
                    ident: ident.to_owned(),
                    include: false,
                    mods: vec![],
                });
                mods.last_mut().expect("mods is not empty")
            }
        }
    }

    let mut mods = Vec::<Mod>::new();
    for path in paths {
        let names = path.split('.').collect::<Vec<&str>>();
        if names.is_empty() {
            continue;
        }
        let mut r#mod = get_or_push(&mut mods, names[0]);
        for name in names.into_iter().skip(1) {
            if name == "rs" {
                r#mod.include = true;
                break;
            } else {
                r#mod = get_or_push(&mut r#mod.mods, name);
            }
        }
    }
    mods
}

fn mods_to_string(mods: &Vec<Mod>) -> String {
    fn dfs(mods: &Vec<Mod>, c: &mut Vec<String>, s: &mut String) {
        let indent = "    ";
        for r#mod in mods {
            s.push_str(&format!(
                "{}pub mod {} {{\n",
                indent.repeat(c.len()),
                r#mod.ident
            ));
            c.push(r#mod.ident.clone());
            if r#mod.include {
                s.push_str(&format!(
                    "{}include!(\"{}.rs\");\n",
                    indent.repeat(c.len()),
                    c.join("."),
                ));
            }
            dfs(&r#mod.mods, c, s);
            c.pop();
            s.push_str(&format!("{}}}\n", indent.repeat(c.len())));
        }
    }

    let mut s = String::new();
    let mut c = vec![];
    dfs(mods, &mut c, &mut s);
    s
}

#[cfg(test)]
mod tests {
    use crate::{mods_from_file_names, mods_to_string, Mod};

    #[test]
    fn test_mods_from_file_names() {
        let paths = [
            "google.firestore.rs",
            "google.firestore.v1.rs",
            "google.firestore.v1beta1.rs",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
        assert_eq!(
            mods_from_file_names(&paths),
            vec![Mod {
                ident: "google".to_owned(),
                include: false,
                mods: vec![Mod {
                    ident: "firestore".to_owned(),
                    include: true,
                    mods: vec![
                        Mod {
                            ident: "v1".to_owned(),
                            include: true,
                            mods: vec![]
                        },
                        Mod {
                            ident: "v1beta1".to_owned(),
                            include: true,
                            mods: vec![]
                        },
                    ]
                }]
            }]
        );
    }

    #[test]
    fn test_mods_to_string() {
        let root = vec![Mod {
            ident: "google".to_owned(),
            include: false,
            mods: vec![Mod {
                ident: "firestore".to_owned(),
                include: true,
                mods: vec![
                    Mod {
                        ident: "v1".to_owned(),
                        include: true,
                        mods: vec![],
                    },
                    Mod {
                        ident: "v1beta1".to_owned(),
                        include: true,
                        mods: vec![],
                    },
                ],
            }],
        }];

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

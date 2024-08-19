use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    str::FromStr as _,
};

use crate::{
    crate_name::CrateName, crate_version::CrateVersion, proto_dir::ProtoDir,
    protobuf_package_name::ProtobufPackageName, sha1hash::Sha1Hash,
};

struct M {
    include: bool,
    modules: BTreeMap<String, M>,
}

pub fn build_crates(
    generated_dir: &Path,
    proto_dir: &ProtoDir,
    old_crate_versions: &BTreeMap<CrateName, CrateVersion>,
    old_package_hashes: &BTreeMap<ProtobufPackageName, Sha1Hash>,
) -> anyhow::Result<BTreeMap<CrateName, CrateVersion>> {
    let mut new_crate_versions = BTreeMap::new();
    let googleapis_tonic_src_dir = generated_dir.join("googleapis-tonic").join("src");
    let all_package_deps = proto_dir.dependencies();
    let emit_package_names = proto_dir.emit_package_names();
    let new_package_hashes = proto_dir.package_hashes();
    let should_update_crates = {
        let mut hash_changed = BTreeSet::new();
        for (package_name, new_hash) in new_package_hashes {
            if old_package_hashes.get(package_name) != Some(new_hash) {
                hash_changed.insert(package_name.to_owned());
            }
        }

        fn dfs(
            memo: &mut BTreeMap<ProtobufPackageName, bool>,
            hash_changed: &BTreeSet<ProtobufPackageName>,
            all_package_deps: &BTreeMap<ProtobufPackageName, BTreeSet<ProtobufPackageName>>,
            package_name: &ProtobufPackageName,
        ) -> bool {
            if let Some(&b) = memo.get(package_name) {
                return b;
            }

            let mut b = hash_changed.contains(package_name);
            for dep in all_package_deps
                .get(package_name)
                .cloned()
                .unwrap_or_default()
            {
                b |= dfs(memo, hash_changed, all_package_deps, &dep);
            }
            memo.insert(package_name.to_owned(), b);
            b
        }

        let mut memo = BTreeMap::new();
        let mut should_update_crates = BTreeSet::new();
        for (package_name, _) in new_package_hashes {
            if dfs(&mut memo, &hash_changed, all_package_deps, package_name) {
                should_update_crates.insert(package_name.to_owned());
            }
        }
        should_update_crates
    };

    for package_name in emit_package_names {
        let package_deps = all_package_deps
            .get(package_name)
            .cloned()
            .unwrap_or_default();
        let crate_name = CrateName::from_package_name(package_name);
        let crate_deps = package_deps
            .iter()
            .filter(|it| emit_package_names.contains(it))
            .map(CrateName::from_package_name)
            .collect::<BTreeSet<CrateName>>();
        let include_package_names = package_deps
            .iter()
            .filter(|it| !emit_package_names.contains(it))
            .chain(std::iter::once(package_name))
            .cloned()
            .collect::<BTreeSet<ProtobufPackageName>>();

        let mut modules = BTreeMap::new();
        for dep in package_deps.iter().chain(std::iter::once(package_name)) {
            let idents = dep
                .to_string()
                .split('.')
                .map(ToString::to_string)
                .collect::<Vec<String>>();
            let idents_len = idents.len();
            idents
                .into_iter()
                .enumerate()
                .fold(&mut modules, |m, (i, s)| {
                    &mut m
                        .entry(s.to_owned())
                        .or_insert(M {
                            include: i == idents_len - 1,
                            modules: BTreeMap::new(),
                        })
                        .modules
                });
        }

        // generated/
        //   googleapis-tonic-{crate_name}/
        //     src/
        //       bytes_btree_map/    ... variant directory
        //         {file_name}
        //       bytes_hash_map/
        //       vec_u8_btree_map/
        //       vec_u8_hash_map/
        //       bytes_btree_map.rs  ... variant file
        //       bytes_hash_map.rs
        //       lib.rs
        //       vec_u8_btree_map.rs
        //       vec_u8_hash_map.rs
        //     Cargo.toml
        let crate_dir = generated_dir.join(crate_name.as_ref());
        fs::create_dir_all(&crate_dir)?;
        let old_crate_version = old_crate_versions
            .get(&crate_name)
            .cloned()
            .unwrap_or_default();
        let new_crate_version = if should_update_crates.contains(package_name) {
            old_crate_version.increment_minor()
        } else {
            old_crate_version
        };
        new_crate_versions.insert(crate_name.clone(), new_crate_version.clone());
        write_cargo_toml(&crate_dir, &crate_name, &crate_deps, &new_crate_version)?;
        let src_dir = crate_dir.join("src");
        for variant in [
            "bytes_btree_map",
            "bytes_hash_map",
            "vec_u8_btree_map",
            "vec_u8_hash_map",
        ] {
            write_variant_dir(
                &googleapis_tonic_src_dir,
                &src_dir,
                variant,
                &include_package_names,
            )?;
            write_variant_file(&src_dir, variant, &modules, &include_package_names)?;
        }
        fs::copy(
            googleapis_tonic_src_dir.join("lib.rs"),
            src_dir.join("lib.rs"),
        )?;
    }
    Ok(new_crate_versions)
}

// crates/googleapis-tonic-{crate_name}/Cargo.toml
fn write_cargo_toml(
    crate_dir: &Path,
    crate_name: &CrateName,
    dep_crate_names: &BTreeSet<CrateName>,
    version: &CrateVersion,
) -> anyhow::Result<()> {
    let cargo_toml_path = crate_dir.join("Cargo.toml");
    let cargo_toml_content = r#"[package]
name = "{CRATE_NAME}"
version = "{VERSION}"
authors = ["bouzuya <m@bouzuya.net>"]
description = "A Google APIs client library generated by tonic-build"
edition = "2021"
keywords = ["api", "gcloud", "gcp", "google", "tonic"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/bouzuya/googleapis-tonic"

[dependencies]
prost = "0.13.1"
prost-types = "0.13.1"
tonic = { version = "0.12.1", default-features = false, features = [
  "codegen",
  "prost",
] }
{DEPENDENCIES}

[lints.clippy]
doc_lazy_continuation = "allow"
large_enum_variant = "allow"
module_inception = "allow"
non_minimal_cfg = "allow"

[lints.rust]
unused_imports = "allow"

[features]
default = ["hash-map", "vec-u8"]
{FEATURES}
"#
    .replace("{CRATE_NAME}", crate_name.as_ref())
    .replace("{VERSION}", &version.to_string())
    .replace(
        "{DEPENDENCIES}",
        &dep_crate_names
            .iter()
            .map(|dep| {
                format!(
                    "{} = {{ version = \"{}\", default-features = false }}",
                    dep, version,
                )
            })
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .replace("{FEATURES}", &{
        ["btree-map", "bytes", "hash-map", "vec-u8"]
            .into_iter()
            .map(|feature| {
                format!(
                    r#"{} = [{}]"#,
                    feature,
                    dep_crate_names
                        .iter()
                        .map(|dep| format!(r#""{}/{}""#, dep, feature))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    });
    fs::write(cargo_toml_path, cargo_toml_content)?;
    Ok(())
}

// crates/googleapis-tonic-{crate_name}/src/{variant}/{file_name}.rs
fn write_variant_dir(
    googleapis_tonic_src_dir: &Path,
    src_dir: &Path,
    variant: &str,
    include_package_names: &BTreeSet<ProtobufPackageName>,
) -> anyhow::Result<()> {
    let variant_dir = src_dir.join(variant);
    fs::create_dir_all(&variant_dir)?;
    for include in include_package_names {
        let include_file_name = format!("{}.rs", package_name_to_module_name(include));
        fs::copy(
            googleapis_tonic_src_dir
                .join(variant)
                .join(&include_file_name),
            variant_dir.join(&include_file_name),
        )?;
    }
    Ok(())
}

// crates/googleapis-tonic-{crate_name}/src/{variant}.rs
fn write_variant_file(
    src_dir: &Path,
    variant: &str,
    modules: &BTreeMap<String, M>,
    include_package_names: &BTreeSet<ProtobufPackageName>,
) -> anyhow::Result<()> {
    let variant_file = src_dir.join(format!("{}.rs", variant));
    let variant_file_content = {
        fn dfs(
            modules: &BTreeMap<String, M>,
            c: &mut Vec<String>,
            s: &mut String,
            variant: &str,
            include_package_names: &BTreeSet<ProtobufPackageName>,
        ) {
            let indent = "    ";
            for (k, m) in modules {
                s.push_str(&format!(
                    "{}pub mod {} {{\n",
                    indent.repeat(c.len()),
                    // FIXME: other keywords
                    if k == "type" {
                        format!("r#{}", k)
                    } else {
                        k.to_owned()
                    }
                ));
                c.push(k.to_owned());
                if m.include {
                    let current_package_name =
                        ProtobufPackageName::from_str(&c.join(".")).expect("valid package name");
                    if include_package_names.contains(&current_package_name) {
                        let include_file_name =
                            format!("{}.rs", package_name_to_module_name(&current_package_name));
                        s.push_str(&format!(
                            "{}include!(\"{}/{}\");\n",
                            indent.repeat(c.len()),
                            variant,
                            include_file_name
                        ));
                    } else {
                        s.push_str(&format!(
                            "{}pub(crate) use googleapis_tonic_{}::{}::*;\n",
                            indent.repeat(c.len()),
                            c.join("_"),
                            c.iter()
                                .map(|it| if it == "type" {
                                    format!("r#{}", it)
                                } else {
                                    it.to_owned()
                                })
                                .collect::<Vec<String>>()
                                .join("::"),
                        ));
                    }
                }
                dfs(&m.modules, c, s, variant, include_package_names);
                c.pop();
                s.push_str(&format!("{}}}\n", indent.repeat(c.len())));
            }
        }

        let mut s = String::new();
        let mut c = vec![];
        dfs(modules, &mut c, &mut s, variant, include_package_names);
        s
    };
    fs::write(variant_file, variant_file_content)?;
    Ok(())
}

fn package_name_to_module_name(package_name: &ProtobufPackageName) -> String {
    package_name
        .to_string()
        .split('.')
        .map(|s| {
            // FIXME: other keywords
            if s == "type" {
                format!("r#{}", s)
            } else {
                s.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join(".")
}

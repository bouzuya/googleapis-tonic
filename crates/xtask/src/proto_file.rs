use std::{collections::BTreeSet, path::PathBuf, str::FromStr};

use anyhow::Context;

use crate::protobuf_package_name::ProtobufPackageName;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProtoFile {
    import_paths: BTreeSet<PathBuf>,
    package_name: ProtobufPackageName,
}

impl ProtoFile {
    pub fn import_paths(&self) -> &BTreeSet<PathBuf> {
        &self.import_paths
    }

    pub fn package_name(&self) -> &ProtobufPackageName {
        &self.package_name
    }
}

impl FromStr for ProtoFile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut import_paths = BTreeSet::new();
        let mut package_name = None;
        for line in s.lines() {
            // <https://protobuf.dev/reference/protobuf/proto3-spec/#package>
            // package = "package" fullIdent ";"
            if line.starts_with("package") {
                let s = line.trim_start_matches("package ").trim_end_matches(";");
                match package_name {
                    None => {
                        package_name = Some(ProtobufPackageName::from_str(s)?);
                    }
                    Some(package_name) => {
                        anyhow::bail!("multiple package declarations: {} and {}", package_name, s)
                    }
                }
            } else if line.starts_with("import") {
                // <https://protobuf.dev/reference/protobuf/proto3-spec/#import_statement>
                // import = "import" [ "weak" | "public" ] strLit ";"
                let s = line
                    .trim_start_matches("import public ")
                    .trim_start_matches("import weak ")
                    .trim_start_matches("import ")
                    .trim_end_matches(";");
                let path = PathBuf::from_str(s.trim_start_matches("\"").trim_end_matches("\""))?;
                import_paths.insert(path);
            }
        }
        let package_name =
            package_name.context(anyhow::anyhow!("package declaration not found"))?;
        Ok(Self {
            import_paths,
            package_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert!(ProtoFile::from_str("").is_err());

        let proto_file = ProtoFile::from_str("package foo;")?;
        assert!(proto_file.import_paths().is_empty());
        assert_eq!(
            proto_file.package_name(),
            &ProtobufPackageName::from_str("foo")?
        );

        let proto_file = ProtoFile::from_str(
            r#"
package foo;

import "bar.proto";
import public "baz.proto";
import weak "foo.proto";
"#,
        )?;
        assert_eq!(proto_file.import_paths(), &{
            let mut set = BTreeSet::new();
            set.insert(PathBuf::from("bar.proto"));
            set.insert(PathBuf::from("baz.proto"));
            set.insert(PathBuf::from("foo.proto"));
            set
        });
        assert_eq!(
            proto_file.package_name(),
            &ProtobufPackageName::from_str("foo")?
        );
        Ok(())
    }
}

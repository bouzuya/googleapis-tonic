use std::str::FromStr;

use anyhow::Context;

use crate::package_name::PackageName;

pub struct ProtoFile {
    package_name: PackageName,
}

impl ProtoFile {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let mut package_name = None;
        for line in content.lines() {
            // <https://protobuf.dev/reference/protobuf/proto3-spec/#package>
            // package = "package" fullIdent ";"
            if line.starts_with("package") {
                let p = line.trim_start_matches("package ").trim_end_matches(";");
                match package_name {
                    None => {
                        package_name = Some(PackageName::from_str(p)?);
                    }
                    Some(package_name) => {
                        anyhow::bail!("multiple package declarations: {} and {}", package_name, p)
                    }
                }
            }
        }
        Ok(Self {
            package_name: package_name.context(anyhow::anyhow!("package declaration not found"))?,
        })
    }

    pub fn package_name(&self) -> &PackageName {
        &self.package_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let proto_file = ProtoFile::parse("package foo;")?;
        assert_eq!(proto_file.package_name(), &PackageName::from_str("foo")?);
        assert!(ProtoFile::parse("").is_err());
        Ok(())
    }
}

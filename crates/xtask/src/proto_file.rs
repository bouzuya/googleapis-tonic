use anyhow::Context;

struct ProtoFile {
    package: String,
}

impl ProtoFile {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let mut package = None;
        for line in content.lines() {
            // package ...;
            if line.starts_with("package") {
                let package_name = line.trim_start_matches("package ").trim_end_matches(";");
                match package {
                    None => {
                        package = Some(package_name.to_owned());
                    }
                    Some(package) => {
                        anyhow::bail!(
                            "multiple package declarations: {} and {}",
                            package,
                            package_name
                        )
                    }
                }
            }
        }
        Ok(Self {
            package: package.context(anyhow::anyhow!("package not found"))?,
        })
    }

    pub fn package(&self) -> &str {
        &self.package
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let proto_file = ProtoFile::parse("package foo;")?;
        assert_eq!(proto_file.package(), "foo");
        assert!(ProtoFile::parse("").is_err());
        Ok(())
    }
}

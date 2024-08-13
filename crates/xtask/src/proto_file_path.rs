use anyhow::Context;
use std::path::Path;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProtoFilePath(Vec<String>);

impl ProtoFilePath {
    pub fn from_absolute_path(absolute_path: &Path, proto_dir: &Path) -> anyhow::Result<Self> {
        anyhow::ensure!(absolute_path.is_absolute(), "not absolute path");
        let mut absolute_path = absolute_path.canonicalize()?;
        anyhow::ensure!(absolute_path.is_file(), "not file");
        anyhow::ensure!(
            absolute_path
                .extension()
                .map(|it| it == "proto")
                .unwrap_or_default(),
            "not .proto file"
        );
        // remove proto extension
        absolute_path.set_extension("");
        let relative_path = absolute_path
            .strip_prefix(proto_dir)
            .context("not in proto_dir")?
            .to_owned();
        let mut parts = vec![];
        for ancestor in relative_path.ancestors() {
            match ancestor.file_name() {
                Some(file_name_os_str) => {
                    let file_name = file_name_os_str.to_str().context("not UTF-8")?;
                    parts.push(file_name.to_owned());
                }
                None => break,
            }
        }
        parts.reverse();
        Ok(Self(parts))
    }

    pub fn from_import_path_str(s: &str) -> anyhow::Result<Self> {
        anyhow::ensure!(s.ends_with(".proto"), "not .proto file");
        let s = s.trim_end_matches(".proto");
        let parts = s.split('/').collect::<Vec<&str>>();
        anyhow::ensure!(parts
            .iter()
            .all(|it| it.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')));
        Ok(Self(
            parts
                .into_iter()
                .map(|it| it.to_owned())
                .collect::<Vec<String>>(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_from_absolute_path() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let root_dir = temp_dir.path();
        let proto_dir = root_dir.join("proto_dir");
        let dir = proto_dir.join("google").join("firestore").join("v1");
        fs::create_dir_all(&dir)?;
        let proto_file = dir.join("common.proto");
        fs::write(&proto_file, "")?;
        let not_proto_file = dir.join("common.PROTO");
        fs::write(&proto_file, "")?;
        let proto_dir2 = root_dir.join("proto_dir2");

        assert_eq!(
            ProtoFilePath::from_absolute_path(&proto_file, &proto_dir)?,
            ProtoFilePath(
                ["google", "firestore", "v1", "common"]
                    .map(|s| s.to_owned())
                    .to_vec()
            )
        );

        // NOT absolute file
        let result = ProtoFilePath::from_absolute_path(
            &PathBuf::from("google/firestore/v1/common.proto"),
            &proto_dir,
        );
        assert!(result.is_err());

        // NOT .proto file
        let result = ProtoFilePath::from_absolute_path(&not_proto_file, &proto_dir);
        assert!(result.is_err());

        // NOT in proto_dir
        let result = ProtoFilePath::from_absolute_path(&proto_file, &proto_dir2);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_from_import_path_str() -> anyhow::Result<()> {
        assert_eq!(
            ProtoFilePath::from_import_path_str("google/firestore/v1/common.proto")?,
            ProtoFilePath(
                ["google", "firestore", "v1", "common"]
                    .map(|s| s.to_owned())
                    .to_vec()
            )
        );

        // NOT .proto file
        let result = ProtoFilePath::from_import_path_str("google/firestore/v1/common.PROTO");
        assert!(result.is_err());

        // NOT ascii_alphanumeric || _
        let result = ProtoFilePath::from_import_path_str("google/firestore/v1/あ.proto");
        assert!(result.is_err());

        // NOT ascii_alphanumeric || _
        let result = ProtoFilePath::from_import_path_str("google/firestore/v1/common.min.proto");
        assert!(result.is_err());
        Ok(())
    }
}

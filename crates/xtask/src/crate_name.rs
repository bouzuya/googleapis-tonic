use crate::protobuf_package_name::ProtobufPackageName;

/// A crate name.
///
/// e.g. `googleapis-tonic-foo-bar-baz`
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateName(String);

impl CrateName {
    pub fn from_package_name(package_name: &ProtobufPackageName) -> Self {
        CrateName(format!(
            "googleapis-tonic-{}",
            package_name
                .to_string()
                .split('.')
                .collect::<Vec<&str>>()
                .join("-")
        ))
    }
}

impl AsRef<str> for CrateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CrateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
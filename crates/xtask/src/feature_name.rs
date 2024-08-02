use std::fmt::Display;

use crate::{bytes_type::BytesType, ident::Ident, map_type::MapType, package_name::PackageName};

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct FeatureName(String);

impl FeatureName {
    pub fn default() -> Self {
        Self("default".to_owned())
    }
}

impl Display for FeatureName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Vec<Ident>> for FeatureName {
    fn from(idents: Vec<Ident>) -> Self {
        Self(
            idents
                .iter()
                .map(Ident::to_string)
                .map(|s| s.replace("r#", ""))
                .collect::<Vec<String>>()
                .join("-"),
        )
    }
}

impl From<BytesType> for FeatureName {
    fn from(bytes_type: BytesType) -> Self {
        match bytes_type {
            BytesType::Bytes => Self("bytes".to_owned()),
            BytesType::VecU8 => Self("vec-u8".to_owned()),
        }
    }
}

impl From<MapType> for FeatureName {
    fn from(map_type: MapType) -> Self {
        match map_type {
            MapType::BTreeMap => Self("btree-map".to_owned()),
            MapType::HashMap => Self("hash-map".to_owned()),
        }
    }
}

impl<'a> From<&'a PackageName> for FeatureName {
    fn from(package_name: &'a PackageName) -> Self {
        Self(
            package_name
                .to_string()
                .split('.')
                .collect::<Vec<&str>>()
                .join("-"),
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MapType {
    BTreeMap,
    HashMap,
}

impl MapType {
    pub fn values() -> &'static [MapType] {
        &[MapType::BTreeMap, MapType::HashMap]
    }

    pub fn as_feature_name(&self) -> &str {
        match self {
            MapType::BTreeMap => "btree-map",
            MapType::HashMap => "hash-map",
        }
    }

    pub fn as_path_part(&self) -> &str {
        match self {
            MapType::BTreeMap => "btree_map",
            MapType::HashMap => "hash_map",
        }
    }
}

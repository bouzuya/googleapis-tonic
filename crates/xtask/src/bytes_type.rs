#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BytesType {
    Bytes,
    VecU8,
}

impl BytesType {
    pub fn values() -> &'static [BytesType] {
        &[BytesType::Bytes, BytesType::VecU8]
    }

    pub fn as_feature_name(&self) -> &str {
        match self {
            BytesType::Bytes => "bytes",
            BytesType::VecU8 => "vec-u8",
        }
    }

    pub fn as_path_part(&self) -> &str {
        match self {
            BytesType::Bytes => "bytes",
            BytesType::VecU8 => "vec_u8",
        }
    }
}

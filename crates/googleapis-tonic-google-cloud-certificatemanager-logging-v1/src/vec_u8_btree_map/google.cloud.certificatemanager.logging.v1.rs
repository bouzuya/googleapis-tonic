// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificatesExpiry {
    #[prost(int64, tag = "1")]
    pub count: i64,
    #[prost(string, repeated, tag = "2")]
    pub certificates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "certificates_expiry::State", tag = "3")]
    pub state: i32,
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CertificatesExpiry`.
pub mod certificates_expiry {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        Unspecified = 0,
        CloseToExpiry = 1,
        Expired = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::CloseToExpiry => "CLOSE_TO_EXPIRY",
                State::Expired => "EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOSE_TO_EXPIRY" => Some(Self::CloseToExpiry),
                "EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
}
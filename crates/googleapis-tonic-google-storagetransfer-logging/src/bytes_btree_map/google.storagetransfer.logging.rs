// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3ObjectMetadata {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub object_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub md5: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3BucketMetadata {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObjectMetadata {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub object_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub md5: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub crc32c: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsBucketMetadata {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureBlobMetadata {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub container: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub blob_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "5")]
    pub md5: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureBlobContainerMetadata {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub container: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PosixFileMetadata {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub crc32c: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpFileMetadata {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub md5: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpManifestMetadata {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetadata {
    #[prost(enumeration = "StorageSystemType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "object_metadata::Metadata", tags = "3, 4, 5, 6, 7")]
    pub metadata: ::core::option::Option<object_metadata::Metadata>,
}
/// Nested message and enum types in `ObjectMetadata`.
pub mod object_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(message, tag = "3")]
        AwsS3Object(super::AwsS3ObjectMetadata),
        #[prost(message, tag = "4")]
        AzureBlob(super::AzureBlobMetadata),
        #[prost(message, tag = "5")]
        GcsObject(super::GcsObjectMetadata),
        #[prost(message, tag = "6")]
        PosixFile(super::PosixFileMetadata),
        #[prost(message, tag = "7")]
        HttpFile(super::HttpFileMetadata),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerMetadata {
    #[prost(enumeration = "StorageSystemType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "container_metadata::Metadata", tags = "3, 4, 5, 6, 7")]
    pub metadata: ::core::option::Option<container_metadata::Metadata>,
}
/// Nested message and enum types in `ContainerMetadata`.
pub mod container_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(message, tag = "3")]
        AwsS3Bucket(super::AwsS3BucketMetadata),
        #[prost(message, tag = "4")]
        AzureBlobContainer(super::AzureBlobContainerMetadata),
        #[prost(message, tag = "5")]
        GcsBucket(super::GcsBucketMetadata),
        #[prost(message, tag = "6")]
        PosixDirectory(super::PosixFileMetadata),
        #[prost(message, tag = "7")]
        HttpManifest(super::HttpManifestMetadata),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferActivityLog {
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
    #[prost(enumeration = "transfer_activity_log::Action", tag = "2")]
    pub action: i32,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<transfer_activity_log::Status>,
    #[prost(message, optional, tag = "4")]
    pub source_container: ::core::option::Option<ContainerMetadata>,
    #[prost(message, optional, tag = "5")]
    pub destination_container: ::core::option::Option<ContainerMetadata>,
    #[prost(message, optional, tag = "6")]
    pub source_object: ::core::option::Option<ObjectMetadata>,
    #[prost(message, optional, tag = "7")]
    pub destination_object: ::core::option::Option<ObjectMetadata>,
    #[prost(message, optional, tag = "8")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `TransferActivityLog`.
pub mod transfer_activity_log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Status {
        #[prost(string, tag = "1")]
        pub status_code: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub error_type: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub error_message: ::prost::alloc::string::String,
    }
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
    pub enum Action {
        Unspecified = 0,
        Find = 1,
        Copy = 2,
        Delete = 3,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::Find => "FIND",
                Action::Copy => "COPY",
                Action::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "FIND" => Some(Self::Find),
                "COPY" => Some(Self::Copy),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StorageSystemType {
    Unspecified = 0,
    AwsS3 = 1,
    AzureBlob = 2,
    Gcs = 3,
    PosixFs = 4,
    Http = 5,
}
impl StorageSystemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StorageSystemType::Unspecified => "STORAGE_SYSTEM_TYPE_UNSPECIFIED",
            StorageSystemType::AwsS3 => "AWS_S3",
            StorageSystemType::AzureBlob => "AZURE_BLOB",
            StorageSystemType::Gcs => "GCS",
            StorageSystemType::PosixFs => "POSIX_FS",
            StorageSystemType::Http => "HTTP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STORAGE_SYSTEM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AWS_S3" => Some(Self::AwsS3),
            "AZURE_BLOB" => Some(Self::AzureBlob),
            "GCS" => Some(Self::Gcs),
            "POSIX_FS" => Some(Self::PosixFs),
            "HTTP" => Some(Self::Http),
            _ => None,
        }
    }
}
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensitiveAction {
    #[prost(string, tag = "1")]
    pub action_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub action_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag = "3")]
    pub affected_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub source_log_ids: ::prost::alloc::vec::Vec<sensitive_action::SourceLogId>,
    #[prost(string, tag = "5")]
    pub learn_more_uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub access: ::core::option::Option<super::super::super::securitycenter::v1::Access>,
}
/// Nested message and enum types in `SensitiveAction`.
pub mod sensitive_action {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourceLogId {
        #[prost(string, tag = "1")]
        pub resource_container: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub log_time: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(string, tag = "3")]
        pub insert_id: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub query_uri: ::prost::alloc::string::String,
    }
}
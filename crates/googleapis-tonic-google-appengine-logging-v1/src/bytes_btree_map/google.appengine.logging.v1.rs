// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogLine {
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(
        enumeration = "super::super::super::logging::r#type::LogSeverity",
        tag = "2"
    )]
    pub severity: i32,
    #[prost(string, tag = "3")]
    pub log_message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub source_location: ::core::option::Option<SourceLocation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceLocation {
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub line: i64,
    #[prost(string, tag = "3")]
    pub function_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceReference {
    #[prost(string, tag = "1")]
    pub repository: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLog {
    #[prost(string, tag = "1")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "37")]
    pub module_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ip: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "8")]
    pub latency: ::core::option::Option<::prost_types::Duration>,
    #[prost(int64, tag = "9")]
    pub mega_cycles: i64,
    #[prost(string, tag = "10")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub resource: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub http_version: ::prost::alloc::string::String,
    #[prost(int32, tag = "13")]
    pub status: i32,
    #[prost(int64, tag = "14")]
    pub response_size: i64,
    #[prost(string, tag = "15")]
    pub referrer: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(string, tag = "40")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub url_map_entry: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub host: ::prost::alloc::string::String,
    #[prost(double, tag = "21")]
    pub cost: f64,
    #[prost(string, tag = "22")]
    pub task_queue_name: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub task_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "24")]
    pub was_loading_request: bool,
    #[prost(message, optional, tag = "25")]
    pub pending_time: ::core::option::Option<::prost_types::Duration>,
    #[prost(int32, tag = "26")]
    pub instance_index: i32,
    #[prost(bool, tag = "27")]
    pub finished: bool,
    #[prost(bool, tag = "42")]
    pub first: bool,
    #[prost(string, tag = "28")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "29")]
    pub line: ::prost::alloc::vec::Vec<LogLine>,
    #[prost(string, tag = "38")]
    pub app_engine_release: ::prost::alloc::string::String,
    #[prost(string, tag = "39")]
    pub trace_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "43")]
    pub trace_sampled: bool,
    #[prost(message, repeated, tag = "41")]
    pub source_reference: ::prost::alloc::vec::Vec<SourceReference>,
}
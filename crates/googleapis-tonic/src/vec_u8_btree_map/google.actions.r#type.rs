// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<super::super::r#type::DateTime>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<super::super::r#type::DateTime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DateRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<super::super::r#type::Date>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<super::super::r#type::Date>,
}
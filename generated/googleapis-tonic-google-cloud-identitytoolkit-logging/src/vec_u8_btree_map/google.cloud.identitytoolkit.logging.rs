// This file is @generated by prost-build.
/// Log of a request to Identitytoolkit. This proto is modeled after
/// google.cloud.audit.AuditLog so that consumers can easily query
/// for requests regardless of whether those requests were logged via
/// Cloud Audit Logging or Identitytoolkit request logging.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLog {
    /// The name of the service method or operation.
    /// For API calls, this should be the name of the API method.
    /// For example,
    ///
    ///      "google.datastore.v1.Datastore.RunQuery"
    ///      "google.logging.v1.LoggingService.DeleteLog"
    #[prost(string, tag = "1")]
    pub method_name: ::prost::alloc::string::String,
    /// The status of the overall operation.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Metadata about the operation.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// The operation request. This may not include all request parameters,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "4")]
    pub request: ::core::option::Option<::prost_types::Struct>,
    /// The operation response. This may not include all response elements,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "5")]
    pub response: ::core::option::Option<::prost_types::Struct>,
    /// The number of items returned from a List or Query API method,
    /// if applicable.
    #[prost(int64, tag = "6")]
    pub num_response_items: i64,
    /// Other service-specific data about the request, response, and other
    /// information associated with the current event.
    #[prost(message, optional, tag = "7")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
}
/// Metadata about the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// The IP address of the caller.
    #[prost(string, tag = "1")]
    pub caller_ip: ::prost::alloc::string::String,
    /// The user agent of the caller.
    /// This information is not authenticated and should be treated
    /// accordingly.
    ///
    /// For example:
    ///
    /// +   `google-api-python-client/1.4.0`:
    ///      The request was made by the Google API client for Python.
    /// +   `Cloud SDK Command Line Tool apitools-client/1.0 gcloud/0.9.62`:
    ///      The request was made by the Google Cloud SDK CLI (gcloud).
    /// +   `AppEngine-Google; (+<http://code.google.com/appengine;> appid:
    ///       s~my-project`:
    ///      The request was made from the `my-project` App Engine app.
    /// NOLINT
    #[prost(string, tag = "2")]
    pub caller_supplied_user_agent: ::prost::alloc::string::String,
}

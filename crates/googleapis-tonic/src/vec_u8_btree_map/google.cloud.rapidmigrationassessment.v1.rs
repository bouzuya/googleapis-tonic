// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestOsScan {
    #[prost(string, tag = "1")]
    pub core_source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VSphereScan {
    #[prost(string, tag = "1")]
    pub core_source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collector {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub service_account: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(int64, tag = "9")]
    pub expected_asset_count: i64,
    #[prost(enumeration = "collector::State", tag = "10")]
    pub state: i32,
    #[prost(string, tag = "11")]
    pub client_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "12")]
    pub guest_os_scan: ::core::option::Option<GuestOsScan>,
    #[prost(message, optional, tag = "13")]
    pub vsphere_scan: ::core::option::Option<VSphereScan>,
    #[prost(int32, tag = "14")]
    pub collection_days: i32,
    #[prost(string, tag = "15")]
    pub eula_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Collector`.
pub mod collector {
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
        Initializing = 1,
        ReadyToUse = 2,
        Registered = 3,
        Active = 4,
        Paused = 5,
        Deleting = 6,
        Decommissioned = 7,
        Error = 8,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Initializing => "STATE_INITIALIZING",
                State::ReadyToUse => "STATE_READY_TO_USE",
                State::Registered => "STATE_REGISTERED",
                State::Active => "STATE_ACTIVE",
                State::Paused => "STATE_PAUSED",
                State::Deleting => "STATE_DELETING",
                State::Decommissioned => "STATE_DECOMMISSIONED",
                State::Error => "STATE_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_INITIALIZING" => Some(Self::Initializing),
                "STATE_READY_TO_USE" => Some(Self::ReadyToUse),
                "STATE_REGISTERED" => Some(Self::Registered),
                "STATE_ACTIVE" => Some(Self::Active),
                "STATE_PAUSED" => Some(Self::Paused),
                "STATE_DELETING" => Some(Self::Deleting),
                "STATE_DECOMMISSIONED" => Some(Self::Decommissioned),
                "STATE_ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(enumeration = "annotation::Type", tag = "5")]
    pub r#type: i32,
}
/// Nested message and enum types in `Annotation`.
pub mod annotation {
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
    pub enum Type {
        Unspecified = 0,
        LegacyExportConsent = 1,
        Qwiklab = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::LegacyExportConsent => "TYPE_LEGACY_EXPORT_CONSENT",
                Type::Qwiklab => "TYPE_QWIKLAB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TYPE_LEGACY_EXPORT_CONSENT" => Some(Self::LegacyExportConsent),
                "TYPE_QWIKLAB" => Some(Self::Qwiklab),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub annotation: ::core::option::Option<Annotation>,
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCollectorRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collector_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub collector: ::core::option::Option<Collector>,
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectorsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub collectors: ::prost::alloc::vec::Vec<Collector>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectorRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectorRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectorRequest {
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    #[prost(message, optional, tag = "2")]
    pub collector: ::core::option::Option<Collector>,
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeCollectorRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterCollectorRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseCollectorRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod rapid_migration_assessment_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources.
    #[derive(Debug, Clone)]
    pub struct RapidMigrationAssessmentClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RapidMigrationAssessmentClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RapidMigrationAssessmentClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            RapidMigrationAssessmentClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Create a Collector to manage the on-prem appliance which collects
        /// information about Customer assets.
        pub async fn create_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCollectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/CreateCollector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "CreateCollector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an Annotation
        pub async fn create_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/CreateAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "CreateAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Annotation.
        pub async fn get_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationRequest>,
        ) -> std::result::Result<tonic::Response<super::Annotation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/GetAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "GetAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Collectors in a given project and location.
        pub async fn list_collectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCollectorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCollectorsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/ListCollectors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "ListCollectors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Collector.
        pub async fn get_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectorRequest>,
        ) -> std::result::Result<tonic::Response<super::Collector>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/GetCollector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "GetCollector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Collector.
        pub async fn update_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/UpdateCollector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "UpdateCollector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Collector - changes state of collector to "Deleting".
        /// Background jobs does final deletion thorugh producer api.
        pub async fn delete_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCollectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/DeleteCollector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "DeleteCollector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resumes the given collector.
        pub async fn resume_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeCollectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/ResumeCollector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "ResumeCollector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Registers the given collector.
        pub async fn register_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterCollectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/RegisterCollector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "RegisterCollector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Pauses the given collector.
        pub async fn pause_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseCollectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/PauseCollector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment",
                        "PauseCollector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
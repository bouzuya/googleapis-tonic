// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildTarget {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Milestone {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub milestone: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub build_version: ::prost::alloc::string::String,
    #[prost(enumeration = "build::BuildStatus", tag = "4")]
    pub status: i32,
    #[prost(enumeration = "build::BuildType", tag = "5")]
    pub r#type: i32,
    #[prost(string, tag = "6")]
    pub branch: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub rw_firmware_version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "8")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Build`.
pub mod build {
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
    pub enum BuildStatus {
        Unspecified = 0,
        Pass = 1,
        Fail = 2,
        Running = 3,
        Aborted = 4,
    }
    impl BuildStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BuildStatus::Unspecified => "BUILD_STATUS_UNSPECIFIED",
                BuildStatus::Pass => "PASS",
                BuildStatus::Fail => "FAIL",
                BuildStatus::Running => "RUNNING",
                BuildStatus::Aborted => "ABORTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUILD_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "PASS" => Some(Self::Pass),
                "FAIL" => Some(Self::Fail),
                "RUNNING" => Some(Self::Running),
                "ABORTED" => Some(Self::Aborted),
                _ => None,
            }
        }
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
    pub enum BuildType {
        Unspecified = 0,
        Release = 1,
        Firmware = 2,
    }
    impl BuildType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BuildType::Unspecified => "BUILD_TYPE_UNSPECIFIED",
                BuildType::Release => "RELEASE",
                BuildType::Firmware => "FIRMWARE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUILD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "RELEASE" => Some(Self::Release),
                "FIRMWARE" => Some(Self::Firmware),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildArtifact {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub build: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub object_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindMostStableBuildRequest {
    #[prost(string, tag = "1")]
    pub build_target: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindMostStableBuildResponse {
    #[prost(message, optional, tag = "1")]
    pub build: ::core::option::Option<Build>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTargetsRequest {
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTargetsResponse {
    #[prost(message, repeated, tag = "1")]
    pub build_targets: ::prost::alloc::vec::Vec<BuildTarget>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    #[prost(message, optional, tag = "6")]
    pub group_by: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsResponse {
    #[prost(message, repeated, tag = "1")]
    pub builds: ::prost::alloc::vec::Vec<Build>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBuildStageStatusRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBuildStageStatusResponse {
    #[prost(bool, tag = "1")]
    pub is_build_staged: bool,
    #[prost(message, optional, tag = "2")]
    pub staged_build_artifact: ::core::option::Option<BuildArtifact>,
    #[prost(message, optional, tag = "3")]
    pub source_build_artifact: ::core::option::Option<BuildArtifact>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildResponse {
    #[prost(message, optional, tag = "1")]
    pub staged_build_artifact: ::core::option::Option<BuildArtifact>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct StageBuildMetadata {
    #[prost(float, tag = "1")]
    pub progress_percent: f32,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod build_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages Chrome OS build services.
    #[derive(Debug, Clone)]
    pub struct BuildServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BuildServiceClient<T>
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
        ) -> BuildServiceClient<InterceptedService<T, F>>
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
            BuildServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all build targets that a user has access to.
        pub async fn list_build_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBuildTargetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBuildTargetsResponse>,
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
                "/google.chromeos.moblab.v1beta1.BuildService/ListBuildTargets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.chromeos.moblab.v1beta1.BuildService",
                        "ListBuildTargets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all models for the given build target.
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModelsResponse>,
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
                "/google.chromeos.moblab.v1beta1.BuildService/ListModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.chromeos.moblab.v1beta1.BuildService",
                        "ListModels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all builds for the given build target and model in descending order
        /// for the milestones and build versions.
        pub async fn list_builds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBuildsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBuildsResponse>,
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
                "/google.chromeos.moblab.v1beta1.BuildService/ListBuilds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.chromeos.moblab.v1beta1.BuildService",
                        "ListBuilds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Checks the stage status for a given build artifact in a partner Google
        /// Cloud Storage bucket.
        pub async fn check_build_stage_status(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckBuildStageStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckBuildStageStatusResponse>,
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
                "/google.chromeos.moblab.v1beta1.BuildService/CheckBuildStageStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.chromeos.moblab.v1beta1.BuildService",
                        "CheckBuildStageStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stages a given build artifact from a internal Google Cloud Storage bucket
        /// to a partner Google Cloud Storage bucket. The stage will be skipped if all
        /// the objects in the partner bucket are the same as in the internal bucket.
        /// Operation
        /// <response:[StageBuildResponse][google.chromeos.moblab.v1beta1.StageBuildResponse],
        ///            metadata:
        ///           [StageBuildMetadata][google.chromeos.moblab.v1beta1.StageBuildMetadata]>
        pub async fn stage_build(
            &mut self,
            request: impl tonic::IntoRequest<super::StageBuildRequest>,
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
                "/google.chromeos.moblab.v1beta1.BuildService/StageBuild",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.chromeos.moblab.v1beta1.BuildService",
                        "StageBuild",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Finds the most stable build for the given build target. The definition of
        /// the most stable build is determined by evaluating the following rule in
        /// order until one is true. If none are true, then there is no stable build
        /// and it will return an empty response.
        ///
        /// Evaluation rules:
        ///   1. Stable channel build with label “Live”
        ///   2. Beta channel build with label “Live”
        ///   3. Dev channel build with label “Live”
        ///   4. Most recent stable channel build with build status Pass
        ///   5. Most recent beta channel build with build status Pass
        ///   6. Most recent dev channel build with build status Pass
        pub async fn find_most_stable_build(
            &mut self,
            request: impl tonic::IntoRequest<super::FindMostStableBuildRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindMostStableBuildResponse>,
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
                "/google.chromeos.moblab.v1beta1.BuildService/FindMostStableBuild",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.chromeos.moblab.v1beta1.BuildService",
                        "FindMostStableBuild",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
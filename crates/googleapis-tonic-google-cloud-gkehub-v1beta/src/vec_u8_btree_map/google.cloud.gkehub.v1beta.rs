// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "3")]
    pub resource_state: ::core::option::Option<FeatureResourceState>,
    #[prost(message, optional, tag = "4")]
    pub spec: ::core::option::Option<CommonFeatureSpec>,
    #[prost(btree_map = "string, message", tag = "5")]
    pub membership_specs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        MembershipFeatureSpec,
    >,
    #[prost(message, optional, tag = "6")]
    pub state: ::core::option::Option<CommonFeatureState>,
    #[prost(btree_map = "string, message", tag = "7")]
    pub membership_states: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        MembershipFeatureState,
    >,
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FeatureResourceState {
    #[prost(enumeration = "feature_resource_state::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `FeatureResourceState`.
pub mod feature_resource_state {
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
        Enabling = 1,
        Active = 2,
        Disabling = 3,
        Updating = 4,
        ServiceUpdating = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabling => "ENABLING",
                State::Active => "ACTIVE",
                State::Disabling => "DISABLING",
                State::Updating => "UPDATING",
                State::ServiceUpdating => "SERVICE_UPDATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLING" => Some(Self::Enabling),
                "ACTIVE" => Some(Self::Active),
                "DISABLING" => Some(Self::Disabling),
                "UPDATING" => Some(Self::Updating),
                "SERVICE_UPDATING" => Some(Self::ServiceUpdating),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureState {
    #[prost(enumeration = "feature_state::Code", tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `FeatureState`.
pub mod feature_state {
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
    pub enum Code {
        Unspecified = 0,
        Ok = 1,
        Warning = 2,
        Error = 3,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Unspecified => "CODE_UNSPECIFIED",
                Code::Ok => "OK",
                Code::Warning => "WARNING",
                Code::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "OK" => Some(Self::Ok),
                "WARNING" => Some(Self::Warning),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonFeatureSpec {
    #[prost(oneof = "common_feature_spec::FeatureSpec", tags = "102")]
    pub feature_spec: ::core::option::Option<common_feature_spec::FeatureSpec>,
}
/// Nested message and enum types in `CommonFeatureSpec`.
pub mod common_feature_spec {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureSpec {
        #[prost(message, tag = "102")]
        Multiclusteringress(super::super::multiclusteringress::v1beta::FeatureSpec),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonFeatureState {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<FeatureState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipFeatureSpec {
    #[prost(oneof = "membership_feature_spec::FeatureSpec", tags = "106, 116")]
    pub feature_spec: ::core::option::Option<membership_feature_spec::FeatureSpec>,
}
/// Nested message and enum types in `MembershipFeatureSpec`.
pub mod membership_feature_spec {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureSpec {
        #[prost(message, tag = "106")]
        Configmanagement(super::super::configmanagement::v1beta::MembershipSpec),
        #[prost(message, tag = "116")]
        Mesh(super::super::servicemesh::v1beta::MembershipSpec),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipFeatureState {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<FeatureState>,
    #[prost(oneof = "membership_feature_state::FeatureState", tags = "100, 104, 106")]
    pub feature_state: ::core::option::Option<membership_feature_state::FeatureState>,
}
/// Nested message and enum types in `MembershipFeatureState`.
pub mod membership_feature_state {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureState {
        #[prost(message, tag = "100")]
        Servicemesh(super::super::servicemesh::v1beta::MembershipState),
        #[prost(message, tag = "104")]
        Metering(super::super::metering::v1beta::MembershipState),
        #[prost(message, tag = "106")]
        Configmanagement(super::super::configmanagement::v1beta::MembershipState),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeaturesRequest {
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
pub struct ListFeaturesResponse {
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Feature>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeatureRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeatureRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub feature_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Feature>,
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeatureRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub force: bool,
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeatureRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Feature>,
    #[prost(string, tag = "4")]
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
    pub status_detail: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod gke_hub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The GKE Hub service handles the registration of many Kubernetes clusters to
    /// Google Cloud, and the management of multi-cluster features over those
    /// clusters.
    ///
    /// The GKE Hub service operates on the following resources:
    ///
    /// * \[Membership\]\[google.cloud.gkehub.v1beta.Membership\]
    /// * \[Feature\]\[google.cloud.gkehub.v1beta.Feature\]
    ///
    /// GKE Hub is currently only available in the global region.
    ///
    /// **Membership management may be non-trivial:** it is recommended to use one
    /// of the Google-provided client libraries or tools where possible when working
    /// with Membership resources.
    #[derive(Debug, Clone)]
    pub struct GkeHubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GkeHubClient<T>
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
        ) -> GkeHubClient<InterceptedService<T, F>>
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
            GkeHubClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Features in a given project and location.
        pub async fn list_features(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeaturesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFeaturesResponse>,
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
                "/google.cloud.gkehub.v1beta.GkeHub/ListFeatures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.gkehub.v1beta.GkeHub", "ListFeatures"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Feature.
        pub async fn get_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeatureRequest>,
        ) -> std::result::Result<tonic::Response<super::Feature>, tonic::Status> {
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
                "/google.cloud.gkehub.v1beta.GkeHub/GetFeature",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.gkehub.v1beta.GkeHub", "GetFeature"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Adds a new Feature.
        pub async fn create_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeatureRequest>,
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
                "/google.cloud.gkehub.v1beta.GkeHub/CreateFeature",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.gkehub.v1beta.GkeHub", "CreateFeature"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Removes a Feature.
        pub async fn delete_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeatureRequest>,
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
                "/google.cloud.gkehub.v1beta.GkeHub/DeleteFeature",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.gkehub.v1beta.GkeHub", "DeleteFeature"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing Feature.
        pub async fn update_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeatureRequest>,
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
                "/google.cloud.gkehub.v1beta.GkeHub/UpdateFeature",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.gkehub.v1beta.GkeHub", "UpdateFeature"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
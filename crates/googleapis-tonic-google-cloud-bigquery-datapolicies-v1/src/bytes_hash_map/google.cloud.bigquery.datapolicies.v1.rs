// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataPolicyRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub data_policy: ::core::option::Option<DataPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataPolicyRequest {
    #[prost(message, optional, tag = "1")]
    pub data_policy: ::core::option::Option<DataPolicy>,
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameDataPolicyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_data_policy_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataPolicyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataPolicyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataPoliciesRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataPoliciesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data_policies: ::prost::alloc::vec::Vec<DataPolicy>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataPolicy {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "data_policy::DataPolicyType", tag = "2")]
    pub data_policy_type: i32,
    #[prost(string, tag = "3")]
    pub data_policy_id: ::prost::alloc::string::String,
    #[prost(oneof = "data_policy::MatchingLabel", tags = "4")]
    pub matching_label: ::core::option::Option<data_policy::MatchingLabel>,
    #[prost(oneof = "data_policy::Policy", tags = "5")]
    pub policy: ::core::option::Option<data_policy::Policy>,
}
/// Nested message and enum types in `DataPolicy`.
pub mod data_policy {
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
    pub enum DataPolicyType {
        Unspecified = 0,
        ColumnLevelSecurityPolicy = 3,
        DataMaskingPolicy = 2,
    }
    impl DataPolicyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataPolicyType::Unspecified => "DATA_POLICY_TYPE_UNSPECIFIED",
                DataPolicyType::ColumnLevelSecurityPolicy => {
                    "COLUMN_LEVEL_SECURITY_POLICY"
                }
                DataPolicyType::DataMaskingPolicy => "DATA_MASKING_POLICY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_POLICY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "COLUMN_LEVEL_SECURITY_POLICY" => Some(Self::ColumnLevelSecurityPolicy),
                "DATA_MASKING_POLICY" => Some(Self::DataMaskingPolicy),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchingLabel {
        #[prost(string, tag = "4")]
        PolicyTag(::prost::alloc::string::String),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Policy {
        #[prost(message, tag = "5")]
        DataMaskingPolicy(super::DataMaskingPolicy),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataMaskingPolicy {
    #[prost(oneof = "data_masking_policy::MaskingExpression", tags = "1, 3")]
    pub masking_expression: ::core::option::Option<
        data_masking_policy::MaskingExpression,
    >,
}
/// Nested message and enum types in `DataMaskingPolicy`.
pub mod data_masking_policy {
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
    pub enum PredefinedExpression {
        Unspecified = 0,
        Sha256 = 3,
        AlwaysNull = 5,
        DefaultMaskingValue = 7,
        LastFourCharacters = 9,
        FirstFourCharacters = 10,
        EmailMask = 12,
        DateYearMask = 13,
    }
    impl PredefinedExpression {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PredefinedExpression::Unspecified => "PREDEFINED_EXPRESSION_UNSPECIFIED",
                PredefinedExpression::Sha256 => "SHA256",
                PredefinedExpression::AlwaysNull => "ALWAYS_NULL",
                PredefinedExpression::DefaultMaskingValue => "DEFAULT_MASKING_VALUE",
                PredefinedExpression::LastFourCharacters => "LAST_FOUR_CHARACTERS",
                PredefinedExpression::FirstFourCharacters => "FIRST_FOUR_CHARACTERS",
                PredefinedExpression::EmailMask => "EMAIL_MASK",
                PredefinedExpression::DateYearMask => "DATE_YEAR_MASK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PREDEFINED_EXPRESSION_UNSPECIFIED" => Some(Self::Unspecified),
                "SHA256" => Some(Self::Sha256),
                "ALWAYS_NULL" => Some(Self::AlwaysNull),
                "DEFAULT_MASKING_VALUE" => Some(Self::DefaultMaskingValue),
                "LAST_FOUR_CHARACTERS" => Some(Self::LastFourCharacters),
                "FIRST_FOUR_CHARACTERS" => Some(Self::FirstFourCharacters),
                "EMAIL_MASK" => Some(Self::EmailMask),
                "DATE_YEAR_MASK" => Some(Self::DateYearMask),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MaskingExpression {
        #[prost(enumeration = "PredefinedExpression", tag = "1")]
        PredefinedExpression(i32),
        #[prost(string, tag = "3")]
        Routine(::prost::alloc::string::String),
    }
}
/// Generated client implementations.
pub mod data_policy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Data Policy Service provides APIs for managing the label-policy bindings.
    #[derive(Debug, Clone)]
    pub struct DataPolicyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataPolicyServiceClient<T>
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
        ) -> DataPolicyServiceClient<InterceptedService<T, F>>
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
            DataPolicyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new data policy under a project with the given `dataPolicyId`
        /// (used as the display name), policy tag, and data policy type.
        pub async fn create_data_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::DataPolicy>, tonic::Status> {
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/CreateDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "CreateDataPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the metadata for an existing data policy. The target data policy
        /// can be specified by the resource name.
        pub async fn update_data_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::DataPolicy>, tonic::Status> {
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/UpdateDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "UpdateDataPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renames the id (display name) of the specified data policy.
        pub async fn rename_data_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameDataPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::DataPolicy>, tonic::Status> {
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/RenameDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "RenameDataPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the data policy specified by its resource name.
        pub async fn delete_data_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataPolicyRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/DeleteDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "DeleteDataPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the data policy specified by its resource name.
        pub async fn get_data_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::DataPolicy>, tonic::Status> {
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/GetDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "GetDataPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all of the data policies in the specified parent project.
        pub async fn list_data_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDataPoliciesResponse>,
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/ListDataPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "ListDataPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM policy for the specified data policy.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the IAM policy for the specified data policy.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the caller's permission on the specified data policy resource.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.bigquery.datapolicies.v1.DataPolicyService/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1.DataPolicyService",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
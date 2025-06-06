// This file is @generated by prost-build.
/// Request message for the CreateDataPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataPolicyRequest {
    /// Required. Resource name of the project that the data policy will belong to. The
    /// format is `projects/{project_number}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The data policy to create. The `name` field does not need to be
    /// provided for the data policy creation.
    #[prost(message, optional, tag = "2")]
    pub data_policy: ::core::option::Option<DataPolicy>,
}
/// Response message for the UpdateDataPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataPolicyRequest {
    /// Required. Update the data policy's metadata.
    ///
    /// The target data policy is determined by the `name` field.
    /// Other fields are updated to the specified values based on the field masks.
    #[prost(message, optional, tag = "1")]
    pub data_policy: ::core::option::Option<DataPolicy>,
    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    /// If not set, defaults to all of the fields that are allowed to update.
    ///
    /// Updates to the `name` and `dataPolicyId` fields are not allowed.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for the DeleteDataPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataPolicyRequest {
    /// Required. Resource name of the data policy to delete. Format is
    /// `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the GetDataPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataPolicyRequest {
    /// Required. Resource name of the requested data policy. Format is
    /// `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the ListDataPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataPoliciesRequest {
    /// Required. Resource name of the project for which to list data policies. Format is
    /// `projects/{project_number}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of data policies to return. Must be a value between 1
    /// and 1000.
    /// If not set, defaults to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `nextPageToken` value returned from a previous list request, if any. If
    /// not set, defaults to an empty string.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListDataPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataPoliciesResponse {
    /// Data policies that belong to the requested project.
    #[prost(message, repeated, tag = "1")]
    pub data_policies: ::prost::alloc::vec::Vec<DataPolicy>,
    /// Token used to retrieve the next page of results, or empty if there are no
    /// more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents the label-policy binding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataPolicy {
    /// Output only. Resource name of this data policy, in the format of
    /// `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of data policy.
    #[prost(enumeration = "data_policy::DataPolicyType", tag = "2")]
    pub data_policy_type: i32,
    /// User-assigned (human readable) ID of the data policy that needs to be
    /// unique within a project. Used as {data_policy_id} in part of the resource
    /// name.
    #[prost(string, tag = "3")]
    pub data_policy_id: ::prost::alloc::string::String,
    /// Label that is bound to this data policy.
    #[prost(oneof = "data_policy::MatchingLabel", tags = "4")]
    pub matching_label: ::core::option::Option<data_policy::MatchingLabel>,
    /// The policy that is bound to this data policy.
    #[prost(oneof = "data_policy::Policy", tags = "5")]
    pub policy: ::core::option::Option<data_policy::Policy>,
}
/// Nested message and enum types in `DataPolicy`.
pub mod data_policy {
    /// A list of supported data policy types.
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
        /// Default value for the data policy type. This should not be used.
        Unspecified = 0,
        /// Used to create a data policy for column-level security, without data
        /// masking.
        ColumnLevelSecurityPolicy = 3,
        /// Used to create a data policy for data masking.
        DataMaskingPolicy = 2,
    }
    impl DataPolicyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "DATA_POLICY_TYPE_UNSPECIFIED",
                Self::ColumnLevelSecurityPolicy => "COLUMN_LEVEL_SECURITY_POLICY",
                Self::DataMaskingPolicy => "DATA_MASKING_POLICY",
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
    /// Label that is bound to this data policy.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchingLabel {
        /// Policy tag resource name, in the format of
        /// `projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}`.
        #[prost(string, tag = "4")]
        PolicyTag(::prost::alloc::string::String),
    }
    /// The policy that is bound to this data policy.
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Policy {
        /// The data masking policy that specifies the data masking rule to use.
        #[prost(message, tag = "5")]
        DataMaskingPolicy(super::DataMaskingPolicy),
    }
}
/// The data masking policy that is used to specify data masking rule.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DataMaskingPolicy {
    /// A masking expression to bind to the data masking rule.
    #[prost(oneof = "data_masking_policy::MaskingExpression", tags = "1")]
    pub masking_expression: ::core::option::Option<
        data_masking_policy::MaskingExpression,
    >,
}
/// Nested message and enum types in `DataMaskingPolicy`.
pub mod data_masking_policy {
    /// The available masking rules. Learn more here:
    /// <https://cloud.google.com/bigquery/docs/column-data-masking-intro#masking_options.>
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
        /// Default, unspecified predefined expression. No masking will take place
        /// since no expression is specified.
        Unspecified = 0,
        /// Masking expression to replace data with SHA-256 hash.
        Sha256 = 3,
        /// Masking expression to replace data with NULLs.
        AlwaysNull = 5,
        /// Masking expression to replace data with their default masking values.
        /// The default masking values for each type listed as below:
        ///
        /// * STRING: ""
        /// * BYTES: b''
        /// * INTEGER: 0
        /// * FLOAT: 0.0
        /// * NUMERIC: 0
        /// * BOOLEAN: FALSE
        /// * TIMESTAMP: 0001-01-01 00:00:00 UTC
        /// * DATE: 0001-01-01
        /// * TIME: 00:00:00
        /// * DATETIME: 0001-01-01T00:00:00
        /// * GEOGRAPHY: POINT(0 0)
        /// * BIGNUMERIC: 0
        /// * ARRAY: \[\]
        /// * STRUCT: NOT_APPLICABLE
        /// * JSON: NULL
        DefaultMaskingValue = 7,
    }
    impl PredefinedExpression {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "PREDEFINED_EXPRESSION_UNSPECIFIED",
                Self::Sha256 => "SHA256",
                Self::AlwaysNull => "ALWAYS_NULL",
                Self::DefaultMaskingValue => "DEFAULT_MASKING_VALUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PREDEFINED_EXPRESSION_UNSPECIFIED" => Some(Self::Unspecified),
                "SHA256" => Some(Self::Sha256),
                "ALWAYS_NULL" => Some(Self::AlwaysNull),
                "DEFAULT_MASKING_VALUE" => Some(Self::DefaultMaskingValue),
                _ => None,
            }
        }
    }
    /// A masking expression to bind to the data masking rule.
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum MaskingExpression {
        /// A predefined masking expression.
        #[prost(enumeration = "PredefinedExpression", tag = "1")]
        PredefinedExpression(i32),
    }
}
/// Generated client implementations.
pub mod data_policy_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Data Policy Service provides APIs for managing the label-policy bindings.
    #[derive(Debug, Clone)]
    pub struct DataPolicyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataPolicyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/CreateDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/UpdateDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
                        "UpdateDataPolicy",
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/DeleteDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/GetDataPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/ListDataPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
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
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.datapolicies.v1beta1.DataPolicyService",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}

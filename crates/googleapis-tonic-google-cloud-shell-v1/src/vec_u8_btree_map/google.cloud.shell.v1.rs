// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub docker_image: ::prost::alloc::string::String,
    #[prost(enumeration = "environment::State", tag = "4")]
    pub state: i32,
    #[prost(string, tag = "12")]
    pub web_host: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub ssh_username: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub ssh_host: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub ssh_port: i32,
    #[prost(string, repeated, tag = "8")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
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
        Suspended = 1,
        Pending = 2,
        Running = 3,
        Deleting = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Suspended => "SUSPENDED",
                State::Pending => "PENDING",
                State::Running => "RUNNING",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUSPENDED" => Some(Self::Suspended),
                "PENDING" => Some(Self::Pending),
                "RUNNING" => Some(Self::Running),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentMetadata {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentMetadata {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartEnvironmentRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeEnvironmentRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub id_token: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AuthorizeEnvironmentResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AuthorizeEnvironmentMetadata {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct StartEnvironmentMetadata {
    #[prost(enumeration = "start_environment_metadata::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `StartEnvironmentMetadata`.
pub mod start_environment_metadata {
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
        Starting = 1,
        UnarchivingDisk = 2,
        AwaitingComputeResources = 4,
        Finished = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Starting => "STARTING",
                State::UnarchivingDisk => "UNARCHIVING_DISK",
                State::AwaitingComputeResources => "AWAITING_COMPUTE_RESOURCES",
                State::Finished => "FINISHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STARTING" => Some(Self::Starting),
                "UNARCHIVING_DISK" => Some(Self::UnarchivingDisk),
                "AWAITING_COMPUTE_RESOURCES" => Some(Self::AwaitingComputeResources),
                "FINISHED" => Some(Self::Finished),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartEnvironmentResponse {
    #[prost(message, optional, tag = "1")]
    pub environment: ::core::option::Option<Environment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPublicKeyRequest {
    #[prost(string, tag = "1")]
    pub environment: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPublicKeyResponse {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AddPublicKeyMetadata {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePublicKeyRequest {
    #[prost(string, tag = "1")]
    pub environment: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RemovePublicKeyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RemovePublicKeyMetadata {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CloudShellErrorDetails {
    #[prost(enumeration = "cloud_shell_error_details::CloudShellErrorCode", tag = "1")]
    pub code: i32,
}
/// Nested message and enum types in `CloudShellErrorDetails`.
pub mod cloud_shell_error_details {
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
    pub enum CloudShellErrorCode {
        Unspecified = 0,
        ImageUnavailable = 1,
        CloudShellDisabled = 2,
        TosViolation = 4,
        QuotaExceeded = 5,
        EnvironmentUnavailable = 6,
    }
    impl CloudShellErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CloudShellErrorCode::Unspecified => "CLOUD_SHELL_ERROR_CODE_UNSPECIFIED",
                CloudShellErrorCode::ImageUnavailable => "IMAGE_UNAVAILABLE",
                CloudShellErrorCode::CloudShellDisabled => "CLOUD_SHELL_DISABLED",
                CloudShellErrorCode::TosViolation => "TOS_VIOLATION",
                CloudShellErrorCode::QuotaExceeded => "QUOTA_EXCEEDED",
                CloudShellErrorCode::EnvironmentUnavailable => "ENVIRONMENT_UNAVAILABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLOUD_SHELL_ERROR_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMAGE_UNAVAILABLE" => Some(Self::ImageUnavailable),
                "CLOUD_SHELL_DISABLED" => Some(Self::CloudShellDisabled),
                "TOS_VIOLATION" => Some(Self::TosViolation),
                "QUOTA_EXCEEDED" => Some(Self::QuotaExceeded),
                "ENVIRONMENT_UNAVAILABLE" => Some(Self::EnvironmentUnavailable),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod cloud_shell_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for interacting with Google Cloud Shell. Each user of Cloud Shell has at
    /// least one environment, which has the ID "default". Environment consists of a
    /// Docker image defining what is installed on the environment and a home
    /// directory containing the user's data that will remain across sessions.
    /// Clients use this API to start and fetch information about their environment,
    /// which can then be used to connect to that environment via a separate SSH
    /// client.
    #[derive(Debug, Clone)]
    pub struct CloudShellServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudShellServiceClient<T>
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
        ) -> CloudShellServiceClient<InterceptedService<T, F>>
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
            CloudShellServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets an environment. Returns NOT_FOUND if the environment does not exist.
        pub async fn get_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Environment>, tonic::Status> {
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
                "/google.cloud.shell.v1.CloudShellService/GetEnvironment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.shell.v1.CloudShellService",
                        "GetEnvironment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Starts an existing environment, allowing clients to connect to it. The
        /// returned operation will contain an instance of StartEnvironmentMetadata in
        /// its metadata field. Users can wait for the environment to start by polling
        /// this operation via GetOperation. Once the environment has finished starting
        /// and is ready to accept connections, the operation will contain a
        /// StartEnvironmentResponse in its response field.
        pub async fn start_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::StartEnvironmentRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/StartEnvironment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.shell.v1.CloudShellService",
                        "StartEnvironment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sends OAuth credentials to a running environment on behalf of a user. When
        /// this completes, the environment will be authorized to run various Google
        /// Cloud command line tools without requiring the user to manually
        /// authenticate.
        pub async fn authorize_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthorizeEnvironmentRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/AuthorizeEnvironment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.shell.v1.CloudShellService",
                        "AuthorizeEnvironment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Adds a public SSH key to an environment, allowing clients with the
        /// corresponding private key to connect to that environment via SSH. If a key
        /// with the same content already exists, this will error with ALREADY_EXISTS.
        pub async fn add_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPublicKeyRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/AddPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.shell.v1.CloudShellService",
                        "AddPublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Removes a public SSH key from an environment. Clients will no longer be
        /// able to connect to the environment using the corresponding private key.
        /// If a key with the same content is not present, this will error with
        /// NOT_FOUND.
        pub async fn remove_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePublicKeyRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/RemovePublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.shell.v1.CloudShellService",
                        "RemovePublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
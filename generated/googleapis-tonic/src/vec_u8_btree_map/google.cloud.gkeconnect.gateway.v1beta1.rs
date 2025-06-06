// This file is @generated by prost-build.
/// A request for connection information for a particular membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateCredentialsRequest {
    /// Required. The Fleet membership resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Whether to force the use of Connect Agent-based transport.
    ///
    /// This will return a configuration that uses Connect Agent as the underlying
    /// transport mechanism for cluster types that would otherwise have used a
    /// different transport. Requires that Connect Agent be installed on the
    /// cluster. Setting this field to false is equivalent to not setting it.
    #[prost(bool, tag = "2")]
    pub force_use_agent: bool,
    /// Optional. The Connect Gateway version to be used in the resulting
    /// configuration.
    ///
    /// Leave this field blank to let the server choose the version (recommended).
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// Optional. The namespace to use in the kubeconfig context.
    ///
    /// If this field is specified, the server will set the `namespace` field in
    /// kubeconfig context. If not specified, the `namespace` field is omitted.
    #[prost(string, tag = "4")]
    pub kubernetes_namespace: ::prost::alloc::string::String,
    /// Optional. The operating system where the kubeconfig will be used.
    #[prost(enumeration = "generate_credentials_request::OperatingSystem", tag = "5")]
    pub operating_system: i32,
}
/// Nested message and enum types in `GenerateCredentialsRequest`.
pub mod generate_credentials_request {
    /// Operating systems requiring specialized kubeconfigs.
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
    pub enum OperatingSystem {
        /// Generates a kubeconfig that works for all operating systems not defined
        /// below.
        Unspecified = 0,
        /// Generates a kubeconfig that is specifically designed to work with
        /// Windows.
        Windows = 1,
    }
    impl OperatingSystem {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "OPERATING_SYSTEM_UNSPECIFIED",
                Self::Windows => "OPERATING_SYSTEM_WINDOWS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATING_SYSTEM_UNSPECIFIED" => Some(Self::Unspecified),
                "OPERATING_SYSTEM_WINDOWS" => Some(Self::Windows),
                _ => None,
            }
        }
    }
}
/// Connection information for a particular membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateCredentialsResponse {
    /// A full YAML kubeconfig in serialized format.
    #[prost(bytes = "vec", tag = "1")]
    pub kubeconfig: ::prost::alloc::vec::Vec<u8>,
    /// The generated URI of the cluster as accessed through the Connect Gateway
    /// API.
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod gateway_control_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// GatewayControl is the control plane API for Connect Gateway.
    #[derive(Debug, Clone)]
    pub struct GatewayControlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GatewayControlClient<T>
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
        ) -> GatewayControlClient<InterceptedService<T, F>>
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
            GatewayControlClient::new(InterceptedService::new(inner, interceptor))
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
        /// GenerateCredentials provides connection information that allows a user to
        /// access the specified membership using Connect Gateway.
        pub async fn generate_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateCredentialsResponse>,
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
                "/google.cloud.gkeconnect.gateway.v1beta1.GatewayControl/GenerateCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.gkeconnect.gateway.v1beta1.GatewayControl",
                        "GenerateCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}

// This file is @generated by prost-build.
/// GeneratePackagesSummaryRequest is the request body for the
/// GeneratePackagesSummary API method. It just takes a single name argument,
/// referring to the resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratePackagesSummaryRequest {
    /// Required. The name of the resource to get a packages summary for in the
    /// form of `projects/\[PROJECT_ID\]/resources/\[RESOURCE_URL\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A summary of the packages found within the given resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackagesSummaryResponse {
    /// The unique URL of the image or the container for which this summary
    /// applies.
    #[prost(string, tag = "1")]
    pub resource_url: ::prost::alloc::string::String,
    /// A listing by license name of each of the licenses and their counts.
    #[prost(message, repeated, tag = "2")]
    pub licenses_summary: ::prost::alloc::vec::Vec<
        packages_summary_response::LicensesSummary,
    >,
}
/// Nested message and enum types in `PackagesSummaryResponse`.
pub mod packages_summary_response {
    /// Per license count
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LicensesSummary {
        /// The license of the package. Note that the format of this value is not
        /// guaranteed. It may be nil, an empty string, a boolean value (A | B), a
        /// differently formed boolean value (A OR B), etc...
        #[prost(string, tag = "1")]
        pub license: ::prost::alloc::string::String,
        /// The number of fixable vulnerabilities associated with this resource.
        #[prost(int64, tag = "2")]
        pub count: i64,
    }
}
/// The request to a call of ExportSBOM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSbomRequest {
    /// Required. The name of the resource in the form of
    /// `projects/\[PROJECT_ID\]/resources/\[RESOURCE_URL\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response from a call to ExportSBOM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSbomResponse {
    /// The name of the discovery occurrence in the form
    /// "projects/{project_id}/occurrences/{OCCURRENCE_ID}
    /// It can be used to track the progression of the SBOM export.
    #[prost(string, tag = "1")]
    pub discovery_occurrence_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod container_analysis_v1_beta1_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Retrieves analysis results of Cloud components such as Docker container
    /// images. The Container Analysis API is an implementation of the
    /// [Grafeas](https://grafeas.io) API.
    ///
    /// Analysis results are stored as a series of occurrences. An `Occurrence`
    /// contains information about a specific analysis instance on a resource. An
    /// occurrence refers to a `Note`. A note contains details describing the
    /// analysis and is generally stored in a separate project, called a `Provider`.
    /// Multiple occurrences can refer to the same note.
    ///
    /// For example, an SSL vulnerability could affect multiple images. In this case,
    /// there would be one note for the vulnerability and an occurrence for each
    /// image with the vulnerability referring to that note.
    #[derive(Debug, Clone)]
    pub struct ContainerAnalysisV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContainerAnalysisV1Beta1Client<T>
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
        ) -> ContainerAnalysisV1Beta1Client<InterceptedService<T, F>>
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
            ContainerAnalysisV1Beta1Client::new(
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
        /// Sets the access control policy on the specified note or occurrence.
        /// Requires `containeranalysis.notes.setIamPolicy` or
        /// `containeranalysis.occurrences.setIamPolicy` permission if the resource is
        /// a note or an occurrence, respectively.
        ///
        /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
        /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
        /// occurrences.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a note or an occurrence resource.
        /// Requires `containeranalysis.notes.setIamPolicy` or
        /// `containeranalysis.occurrences.setIamPolicy` permission if the resource is
        /// a note or occurrence, respectively.
        ///
        /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
        /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
        /// occurrences.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the permissions that a caller has on the specified note or
        /// occurrence. Requires list permission on the project (for example,
        /// `containeranalysis.notes.list`).
        ///
        /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
        /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
        /// occurrences.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a summary of the packages within a given resource.
        pub async fn generate_packages_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::GeneratePackagesSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PackagesSummaryResponse>,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GeneratePackagesSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "GeneratePackagesSummary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates an SBOM and other dependency information for the given resource.
        pub async fn export_sbom(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportSbomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportSbomResponse>,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/ExportSBOM",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "ExportSBOM",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}

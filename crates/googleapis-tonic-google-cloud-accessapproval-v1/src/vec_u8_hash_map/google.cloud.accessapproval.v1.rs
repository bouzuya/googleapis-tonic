// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLocations {
    #[prost(string, tag = "1")]
    pub principal_office_country: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub principal_physical_location_country: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessReason {
    #[prost(enumeration = "access_reason::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub detail: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AccessReason`.
pub mod access_reason {
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
        CustomerInitiatedSupport = 1,
        GoogleInitiatedService = 2,
        GoogleInitiatedReview = 3,
        ThirdPartyDataRequest = 4,
        GoogleResponseToProductionAlert = 5,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::CustomerInitiatedSupport => "CUSTOMER_INITIATED_SUPPORT",
                Type::GoogleInitiatedService => "GOOGLE_INITIATED_SERVICE",
                Type::GoogleInitiatedReview => "GOOGLE_INITIATED_REVIEW",
                Type::ThirdPartyDataRequest => "THIRD_PARTY_DATA_REQUEST",
                Type::GoogleResponseToProductionAlert => {
                    "GOOGLE_RESPONSE_TO_PRODUCTION_ALERT"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CUSTOMER_INITIATED_SUPPORT" => Some(Self::CustomerInitiatedSupport),
                "GOOGLE_INITIATED_SERVICE" => Some(Self::GoogleInitiatedService),
                "GOOGLE_INITIATED_REVIEW" => Some(Self::GoogleInitiatedReview),
                "THIRD_PARTY_DATA_REQUEST" => Some(Self::ThirdPartyDataRequest),
                "GOOGLE_RESPONSE_TO_PRODUCTION_ALERT" => {
                    Some(Self::GoogleResponseToProductionAlert)
                }
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "signature_info::VerificationInfo", tags = "2, 3")]
    pub verification_info: ::core::option::Option<signature_info::VerificationInfo>,
}
/// Nested message and enum types in `SignatureInfo`.
pub mod signature_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VerificationInfo {
        #[prost(string, tag = "2")]
        GooglePublicKeyPem(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        CustomerKmsKeyVersion(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveDecision {
    #[prost(message, optional, tag = "1")]
    pub approve_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub invalidate_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub signature_info: ::core::option::Option<SignatureInfo>,
    #[prost(bool, tag = "5")]
    pub auto_approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DismissDecision {
    #[prost(message, optional, tag = "1")]
    pub dismiss_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "2")]
    pub implicit: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ResourceProperties {
    #[prost(bool, tag = "1")]
    pub excludes_descendants: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApprovalRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub requested_resource_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub requested_resource_properties: ::core::option::Option<ResourceProperties>,
    #[prost(message, optional, tag = "3")]
    pub requested_reason: ::core::option::Option<AccessReason>,
    #[prost(message, optional, tag = "4")]
    pub requested_locations: ::core::option::Option<AccessLocations>,
    #[prost(message, optional, tag = "5")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub requested_expiration: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "approval_request::Decision", tags = "7, 8")]
    pub decision: ::core::option::Option<approval_request::Decision>,
}
/// Nested message and enum types in `ApprovalRequest`.
pub mod approval_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Decision {
        #[prost(message, tag = "7")]
        Approve(super::ApproveDecision),
        #[prost(message, tag = "8")]
        Dismiss(super::DismissDecision),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnrolledService {
    #[prost(string, tag = "1")]
    pub cloud_product: ::prost::alloc::string::String,
    #[prost(enumeration = "EnrollmentLevel", tag = "2")]
    pub enrollment_level: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessApprovalSettings {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub notification_emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub enrolled_services: ::prost::alloc::vec::Vec<EnrolledService>,
    #[prost(bool, tag = "4")]
    pub enrolled_ancestor: bool,
    #[prost(string, tag = "6")]
    pub active_key_version: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub ancestor_has_active_key_version: bool,
    #[prost(bool, tag = "8")]
    pub invalid_key_version: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessApprovalServiceAccount {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account_email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApprovalRequestsMessage {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApprovalRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub approval_requests: ::prost::alloc::vec::Vec<ApprovalRequest>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApprovalRequestMessage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveApprovalRequestMessage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DismissApprovalRequestMessage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidateApprovalRequestMessage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccessApprovalSettingsMessage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccessApprovalSettingsMessage {
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<AccessApprovalSettings>,
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccessApprovalSettingsMessage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccessApprovalServiceAccountMessage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnrollmentLevel {
    Unspecified = 0,
    BlockAll = 1,
}
impl EnrollmentLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EnrollmentLevel::Unspecified => "ENROLLMENT_LEVEL_UNSPECIFIED",
            EnrollmentLevel::BlockAll => "BLOCK_ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENROLLMENT_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "BLOCK_ALL" => Some(Self::BlockAll),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod access_approval_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This API allows a customer to manage accesses to cloud resources by
    /// Google personnel. It defines the following resource model:
    ///
    /// * The API has a collection of
    ///  \[ApprovalRequest\]\[google.cloud.accessapproval.v1.ApprovalRequest\]
    ///  resources, named `approvalRequests/{approval_request}`
    /// * The API has top-level settings per Project/Folder/Organization, named
    ///  `accessApprovalSettings`
    ///
    /// The service also periodically emails a list of recipients, defined at the
    /// Project/Folder/Organization level in the accessApprovalSettings, when there
    /// is a pending ApprovalRequest for them to act on. The ApprovalRequests can
    /// also optionally be published to a Pub/Sub topic owned by the customer
    /// (contact support if you would like to enable Pub/Sub notifications).
    ///
    /// ApprovalRequests can be approved or dismissed. Google personnel can only
    /// access the indicated resource or resources if the request is approved
    /// (subject to some exclusions:
    /// https://cloud.google.com/access-approval/docs/overview#exclusions).
    ///
    /// Note: Using Access Approval functionality will mean that Google may not be
    /// able to meet the SLAs for your chosen products, as any support response times
    /// may be dramatically increased. As such the SLAs do not apply to any service
    /// disruption to the extent impacted by Customer's use of Access Approval. Do
    /// not enable Access Approval for projects where you may require high service
    /// availability and rapid response by Google Cloud Support.
    ///
    /// After a request is approved or dismissed, no further action may be taken on
    /// it. Requests with the requested_expiration in the past or with no activity
    /// for 14 days are considered dismissed. When an approval expires, the request
    /// is considered dismissed.
    ///
    /// If a request is not approved or dismissed, we call it pending.
    #[derive(Debug, Clone)]
    pub struct AccessApprovalClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccessApprovalClient<T>
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
        ) -> AccessApprovalClient<InterceptedService<T, F>>
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
            AccessApprovalClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists approval requests associated with a project, folder, or organization.
        /// Approval requests can be filtered by state (pending, active, dismissed).
        /// The order is reverse chronological.
        pub async fn list_approval_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApprovalRequestsMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ListApprovalRequestsResponse>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/ListApprovalRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "ListApprovalRequests",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an approval request. Returns NOT_FOUND if the request does not exist.
        pub async fn get_approval_request(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApprovalRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ApprovalRequest>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/GetApprovalRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "GetApprovalRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Approves a request and returns the updated ApprovalRequest.
        ///
        /// Returns NOT_FOUND if the request does not exist. Returns
        /// FAILED_PRECONDITION if the request exists but is not in a pending state.
        pub async fn approve_approval_request(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveApprovalRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ApprovalRequest>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/ApproveApprovalRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "ApproveApprovalRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Dismisses a request. Returns the updated ApprovalRequest.
        ///
        /// NOTE: This does not deny access to the resource if another request has been
        /// made and approved. It is equivalent in effect to ignoring the request
        /// altogether.
        ///
        /// Returns NOT_FOUND if the request does not exist.
        ///
        /// Returns FAILED_PRECONDITION if the request exists but is not in a pending
        /// state.
        pub async fn dismiss_approval_request(
            &mut self,
            request: impl tonic::IntoRequest<super::DismissApprovalRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ApprovalRequest>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/DismissApprovalRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "DismissApprovalRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Invalidates an existing ApprovalRequest. Returns the updated
        /// ApprovalRequest.
        ///
        /// NOTE: This does not deny access to the resource if another request has been
        /// made and approved. It only invalidates a single approval.
        ///
        /// Returns FAILED_PRECONDITION if the request exists but is not in an approved
        /// state.
        pub async fn invalidate_approval_request(
            &mut self,
            request: impl tonic::IntoRequest<super::InvalidateApprovalRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ApprovalRequest>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/InvalidateApprovalRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "InvalidateApprovalRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the settings associated with a project, folder, or organization.
        pub async fn get_access_approval_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccessApprovalSettingsMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AccessApprovalSettings>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/GetAccessApprovalSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "GetAccessApprovalSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings associated with a project, folder, or organization.
        /// Settings to update are determined by the value of field_mask.
        pub async fn update_access_approval_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccessApprovalSettingsMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AccessApprovalSettings>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/UpdateAccessApprovalSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "UpdateAccessApprovalSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the settings associated with a project, folder, or organization.
        /// This will have the effect of disabling Access Approval for the project,
        /// folder, or organization, but only if all ancestors also have Access
        /// Approval disabled. If Access Approval is enabled at a higher level of the
        /// hierarchy, then Access Approval will still be enabled at this level as
        /// the settings are inherited.
        pub async fn delete_access_approval_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccessApprovalSettingsMessage>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/DeleteAccessApprovalSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "DeleteAccessApprovalSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the service account that is used by Access Approval to access KMS
        /// keys for signing approved approval requests.
        pub async fn get_access_approval_service_account(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetAccessApprovalServiceAccountMessage,
            >,
        ) -> std::result::Result<
            tonic::Response<super::AccessApprovalServiceAccount>,
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
                "/google.cloud.accessapproval.v1.AccessApproval/GetAccessApprovalServiceAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.accessapproval.v1.AccessApproval",
                        "GetAccessApprovalServiceAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
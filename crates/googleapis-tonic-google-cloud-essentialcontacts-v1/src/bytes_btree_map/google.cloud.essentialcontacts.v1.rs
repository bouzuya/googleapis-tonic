// This file is @generated by prost-build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationCategory {
    Unspecified = 0,
    All = 2,
    Suspension = 3,
    Security = 5,
    Technical = 6,
    Billing = 7,
    Legal = 8,
    ProductUpdates = 9,
    TechnicalIncidents = 10,
}
impl NotificationCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationCategory::Unspecified => "NOTIFICATION_CATEGORY_UNSPECIFIED",
            NotificationCategory::All => "ALL",
            NotificationCategory::Suspension => "SUSPENSION",
            NotificationCategory::Security => "SECURITY",
            NotificationCategory::Technical => "TECHNICAL",
            NotificationCategory::Billing => "BILLING",
            NotificationCategory::Legal => "LEGAL",
            NotificationCategory::ProductUpdates => "PRODUCT_UPDATES",
            NotificationCategory::TechnicalIncidents => "TECHNICAL_INCIDENTS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "ALL" => Some(Self::All),
            "SUSPENSION" => Some(Self::Suspension),
            "SECURITY" => Some(Self::Security),
            "TECHNICAL" => Some(Self::Technical),
            "BILLING" => Some(Self::Billing),
            "LEGAL" => Some(Self::Legal),
            "PRODUCT_UPDATES" => Some(Self::ProductUpdates),
            "TECHNICAL_INCIDENTS" => Some(Self::TechnicalIncidents),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ValidationState {
    Unspecified = 0,
    Valid = 1,
    Invalid = 2,
}
impl ValidationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ValidationState::Unspecified => "VALIDATION_STATE_UNSPECIFIED",
            ValidationState::Valid => "VALID",
            ValidationState::Invalid => "INVALID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VALIDATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "VALID" => Some(Self::Valid),
            "INVALID" => Some(Self::Invalid),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(enumeration = "NotificationCategory", repeated, packed = "false", tag = "3")]
    pub notification_category_subscriptions: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "4")]
    pub language_tag: ::prost::alloc::string::String,
    #[prost(enumeration = "ValidationState", tag = "8")]
    pub validation_state: i32,
    #[prost(message, optional, tag = "9")]
    pub validate_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContactsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContactsResponse {
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContactRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteContactRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContactRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contact: ::core::option::Option<Contact>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContactRequest {
    #[prost(message, optional, tag = "2")]
    pub contact: ::core::option::Option<Contact>,
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeContactsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(enumeration = "NotificationCategory", repeated, tag = "6")]
    pub notification_categories: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeContactsResponse {
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendTestMessageRequest {
    #[prost(string, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub resource: ::prost::alloc::string::String,
    #[prost(enumeration = "NotificationCategory", tag = "3")]
    pub notification_category: i32,
}
/// Generated client implementations.
pub mod essential_contacts_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages contacts for important Google Cloud notifications.
    #[derive(Debug, Clone)]
    pub struct EssentialContactsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EssentialContactsServiceClient<T>
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
        ) -> EssentialContactsServiceClient<InterceptedService<T, F>>
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
            EssentialContactsServiceClient::new(
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
        /// Adds a new contact for a resource.
        pub async fn create_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContactRequest>,
        ) -> std::result::Result<tonic::Response<super::Contact>, tonic::Status> {
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
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/CreateContact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.essentialcontacts.v1.EssentialContactsService",
                        "CreateContact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a contact.
        /// Note: A contact's email address cannot be changed.
        pub async fn update_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContactRequest>,
        ) -> std::result::Result<tonic::Response<super::Contact>, tonic::Status> {
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
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/UpdateContact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.essentialcontacts.v1.EssentialContactsService",
                        "UpdateContact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the contacts that have been set on a resource.
        pub async fn list_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactsResponse>,
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
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/ListContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.essentialcontacts.v1.EssentialContactsService",
                        "ListContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single contact.
        pub async fn get_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContactRequest>,
        ) -> std::result::Result<tonic::Response<super::Contact>, tonic::Status> {
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
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/GetContact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.essentialcontacts.v1.EssentialContactsService",
                        "GetContact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a contact.
        pub async fn delete_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContactRequest>,
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
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/DeleteContact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.essentialcontacts.v1.EssentialContactsService",
                        "DeleteContact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all contacts for the resource that are subscribed to the
        /// specified notification categories, including contacts inherited from
        /// any parent resources.
        pub async fn compute_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ComputeContactsResponse>,
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
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/ComputeContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.essentialcontacts.v1.EssentialContactsService",
                        "ComputeContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Allows a contact admin to send a test message to contact to verify that it
        /// has been configured correctly.
        pub async fn send_test_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SendTestMessageRequest>,
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
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/SendTestMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.essentialcontacts.v1.EssentialContactsService",
                        "SendTestMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
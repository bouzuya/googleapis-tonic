// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub subject: ::core::option::Option<Subject>,
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "NotificationType", tag = "12")]
    pub notification_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    #[prost(string, tag = "1")]
    pub en_text: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub localized_text: ::prost::alloc::string::String,
    #[prost(enumeration = "LocalizationState", tag = "3")]
    pub localization_state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<Text>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<message::Body>,
    #[prost(message, repeated, tag = "2")]
    pub attachments: ::prost::alloc::vec::Vec<Attachment>,
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub localization_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Body {
        #[prost(message, optional, tag = "1")]
        pub text: ::core::option::Option<super::Text>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attachment {
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(oneof = "attachment::Data", tags = "2")]
    pub data: ::core::option::Option<attachment::Data>,
}
/// Nested message and enum types in `Attachment`.
pub mod attachment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "2")]
        Csv(super::Csv),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Csv {
    #[prost(string, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub data_rows: ::prost::alloc::vec::Vec<csv::CsvRow>,
}
/// Nested message and enum types in `Csv`.
pub mod csv {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CsvRow {
        #[prost(string, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(enumeration = "NotificationView", tag = "4")]
    pub view: i32,
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "2")]
    pub notification_settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        NotificationSettings,
    >,
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct NotificationSettings {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<Settings>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationView {
    Unspecified = 0,
    Basic = 1,
    Full = 2,
}
impl NotificationView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationView::Unspecified => "NOTIFICATION_VIEW_UNSPECIFIED",
            NotificationView::Basic => "BASIC",
            NotificationView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocalizationState {
    Unspecified = 0,
    NotApplicable = 1,
    Pending = 2,
    Completed = 3,
}
impl LocalizationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocalizationState::Unspecified => "LOCALIZATION_STATE_UNSPECIFIED",
            LocalizationState::NotApplicable => "LOCALIZATION_STATE_NOT_APPLICABLE",
            LocalizationState::Pending => "LOCALIZATION_STATE_PENDING",
            LocalizationState::Completed => "LOCALIZATION_STATE_COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCALIZATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "LOCALIZATION_STATE_NOT_APPLICABLE" => Some(Self::NotApplicable),
            "LOCALIZATION_STATE_PENDING" => Some(Self::Pending),
            "LOCALIZATION_STATE_COMPLETED" => Some(Self::Completed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationType {
    Unspecified = 0,
    SecurityPrivacyAdvisory = 1,
    SensitiveActions = 2,
    SecurityMsa = 3,
    ThreatHorizons = 4,
}
impl NotificationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationType::Unspecified => "NOTIFICATION_TYPE_UNSPECIFIED",
            NotificationType::SecurityPrivacyAdvisory => {
                "NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY"
            }
            NotificationType::SensitiveActions => "NOTIFICATION_TYPE_SENSITIVE_ACTIONS",
            NotificationType::SecurityMsa => "NOTIFICATION_TYPE_SECURITY_MSA",
            NotificationType::ThreatHorizons => "NOTIFICATION_TYPE_THREAT_HORIZONS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY" => {
                Some(Self::SecurityPrivacyAdvisory)
            }
            "NOTIFICATION_TYPE_SENSITIVE_ACTIONS" => Some(Self::SensitiveActions),
            "NOTIFICATION_TYPE_SECURITY_MSA" => Some(Self::SecurityMsa),
            "NOTIFICATION_TYPE_THREAT_HORIZONS" => Some(Self::ThreatHorizons),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod advisory_notifications_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage Security and Privacy Notifications.
    #[derive(Debug, Clone)]
    pub struct AdvisoryNotificationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdvisoryNotificationsServiceClient<T>
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
        ) -> AdvisoryNotificationsServiceClient<InterceptedService<T, F>>
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
            AdvisoryNotificationsServiceClient::new(
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
        /// Lists notifications under a given parent.
        pub async fn list_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationsResponse>,
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/ListNotifications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "ListNotifications",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a notification.
        pub async fn get_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationRequest>,
        ) -> std::result::Result<tonic::Response<super::Notification>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/GetNotification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "GetNotification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get notification settings.
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingsRequest>,
        ) -> std::result::Result<tonic::Response<super::Settings>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/GetSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "GetSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update notification settings.
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
        ) -> std::result::Result<tonic::Response<super::Settings>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/UpdateSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "UpdateSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
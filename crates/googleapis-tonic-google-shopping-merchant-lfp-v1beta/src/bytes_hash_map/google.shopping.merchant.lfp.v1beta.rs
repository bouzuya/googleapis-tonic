// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LfpStore {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub target_account: i64,
    #[prost(string, tag = "3")]
    pub store_code: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub store_address: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub store_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub website_uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub gcid_category: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub place_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "lfp_store::StoreMatchingState", tag = "10")]
    pub matching_state: i32,
    #[prost(string, optional, tag = "11")]
    pub matching_state_hint: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `LfpStore`.
pub mod lfp_store {
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
    pub enum StoreMatchingState {
        Unspecified = 0,
        Matched = 1,
        Failed = 2,
    }
    impl StoreMatchingState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StoreMatchingState::Unspecified => "STORE_MATCHING_STATE_UNSPECIFIED",
                StoreMatchingState::Matched => "STORE_MATCHING_STATE_MATCHED",
                StoreMatchingState::Failed => "STORE_MATCHING_STATE_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STORE_MATCHING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STORE_MATCHING_STATE_MATCHED" => Some(Self::Matched),
                "STORE_MATCHING_STATE_FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLfpStoreRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertLfpStoreRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub lfp_store: ::core::option::Option<LfpStore>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLfpStoreRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLfpStoresRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub target_account: i64,
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLfpStoresResponse {
    #[prost(message, repeated, tag = "1")]
    pub lfp_stores: ::prost::alloc::vec::Vec<LfpStore>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod lfp_store_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for a [LFP
    /// partner](https://support.google.com/merchants/answer/7676652) to submit local
    /// stores for a merchant.
    #[derive(Debug, Clone)]
    pub struct LfpStoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LfpStoreServiceClient<T>
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
        ) -> LfpStoreServiceClient<InterceptedService<T, F>>
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
            LfpStoreServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves information about a store.
        pub async fn get_lfp_store(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLfpStoreRequest>,
        ) -> std::result::Result<tonic::Response<super::LfpStore>, tonic::Status> {
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
                "/google.shopping.merchant.lfp.v1beta.LfpStoreService/GetLfpStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.lfp.v1beta.LfpStoreService",
                        "GetLfpStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Inserts a store for the target merchant. If the store with the same store
        /// code already exists, it will be replaced.
        pub async fn insert_lfp_store(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertLfpStoreRequest>,
        ) -> std::result::Result<tonic::Response<super::LfpStore>, tonic::Status> {
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
                "/google.shopping.merchant.lfp.v1beta.LfpStoreService/InsertLfpStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.lfp.v1beta.LfpStoreService",
                        "InsertLfpStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a store for a target merchant.
        pub async fn delete_lfp_store(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLfpStoreRequest>,
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
                "/google.shopping.merchant.lfp.v1beta.LfpStoreService/DeleteLfpStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.lfp.v1beta.LfpStoreService",
                        "DeleteLfpStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the stores of the target merchant, specified by the filter in
        /// `ListLfpStoresRequest`.
        pub async fn list_lfp_stores(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLfpStoresRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLfpStoresResponse>,
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
                "/google.shopping.merchant.lfp.v1beta.LfpStoreService/ListLfpStores",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.lfp.v1beta.LfpStoreService",
                        "ListLfpStores",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LfpSale {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub target_account: i64,
    #[prost(string, tag = "3")]
    pub store_code: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub region_code: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub content_language: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub gtin: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    #[prost(int64, tag = "9")]
    pub quantity: i64,
    #[prost(message, optional, tag = "10")]
    pub sale_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, optional, tag = "11")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub feed_label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertLfpSaleRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub lfp_sale: ::core::option::Option<LfpSale>,
}
/// Generated client implementations.
pub mod lfp_sale_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for a [LFP
    /// partner](https://support.google.com/merchants/answer/7676652) to submit sales
    /// data for a merchant.
    #[derive(Debug, Clone)]
    pub struct LfpSaleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LfpSaleServiceClient<T>
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
        ) -> LfpSaleServiceClient<InterceptedService<T, F>>
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
            LfpSaleServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Inserts a `LfpSale` for the given merchant.
        pub async fn insert_lfp_sale(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertLfpSaleRequest>,
        ) -> std::result::Result<tonic::Response<super::LfpSale>, tonic::Status> {
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
                "/google.shopping.merchant.lfp.v1beta.LfpSaleService/InsertLfpSale",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.lfp.v1beta.LfpSaleService",
                        "InsertLfpSale",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LfpInventory {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub target_account: i64,
    #[prost(string, tag = "3")]
    pub store_code: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub region_code: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub content_language: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "7")]
    pub gtin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    #[prost(string, tag = "9")]
    pub availability: ::prost::alloc::string::String,
    #[prost(int64, optional, tag = "10")]
    pub quantity: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "11")]
    pub collection_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, optional, tag = "12")]
    pub pickup_method: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub pickup_sla: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub feed_label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertLfpInventoryRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub lfp_inventory: ::core::option::Option<LfpInventory>,
}
/// Generated client implementations.
pub mod lfp_inventory_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for a [LFP
    /// partner](https://support.google.com/merchants/answer/7676652) to submit local
    /// inventories for a merchant.
    #[derive(Debug, Clone)]
    pub struct LfpInventoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LfpInventoryServiceClient<T>
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
        ) -> LfpInventoryServiceClient<InterceptedService<T, F>>
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
            LfpInventoryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Inserts a `LfpInventory` resource for the given target merchant account. If
        /// the resource already exists, it will be replaced. The inventory
        /// automatically expires after 30 days.
        pub async fn insert_lfp_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertLfpInventoryRequest>,
        ) -> std::result::Result<tonic::Response<super::LfpInventory>, tonic::Status> {
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
                "/google.shopping.merchant.lfp.v1beta.LfpInventoryService/InsertLfpInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.lfp.v1beta.LfpInventoryService",
                        "InsertLfpInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
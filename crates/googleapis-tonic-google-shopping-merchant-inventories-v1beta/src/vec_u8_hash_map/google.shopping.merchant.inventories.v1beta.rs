// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalInventory {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub account: i64,
    #[prost(string, tag = "3")]
    pub store_code: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    #[prost(message, optional, tag = "5")]
    pub sale_price: ::core::option::Option<super::super::super::r#type::Price>,
    #[prost(message, optional, tag = "6")]
    pub sale_price_effective_date: ::core::option::Option<
        super::super::super::super::r#type::Interval,
    >,
    #[prost(string, optional, tag = "7")]
    pub availability: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "8")]
    pub quantity: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub pickup_method: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub pickup_sla: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub instore_product_location: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "12")]
    pub custom_attributes: ::prost::alloc::vec::Vec<
        super::super::super::r#type::CustomAttribute,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocalInventoriesRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocalInventoriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub local_inventories: ::prost::alloc::vec::Vec<LocalInventory>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertLocalInventoryRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub local_inventory: ::core::option::Option<LocalInventory>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocalInventoryRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod local_inventory_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage local inventory for products
    #[derive(Debug, Clone)]
    pub struct LocalInventoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LocalInventoryServiceClient<T>
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
        ) -> LocalInventoryServiceClient<InterceptedService<T, F>>
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
            LocalInventoryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the `LocalInventory` resources for the given product in your merchant
        /// account. The response might contain fewer items than specified by
        /// `pageSize`. If `pageToken` was returned in previous request, it can be used
        /// to obtain additional results.
        ///
        /// `LocalInventory` resources are listed per product for a given account.
        pub async fn list_local_inventories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocalInventoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLocalInventoriesResponse>,
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
                "/google.shopping.merchant.inventories.v1beta.LocalInventoryService/ListLocalInventories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.LocalInventoryService",
                        "ListLocalInventories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Inserts a `LocalInventory` resource to a product in your merchant
        /// account.
        ///
        /// Replaces the full `LocalInventory` resource if an entry with the same
        /// \[`storeCode`\]\[google.shopping.merchant.inventories.v1beta.LocalInventory.store_code\]
        /// already exists for the product.
        ///
        /// It might take up to 30 minutes for the new or updated `LocalInventory`
        /// resource to appear in products.
        pub async fn insert_local_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertLocalInventoryRequest>,
        ) -> std::result::Result<tonic::Response<super::LocalInventory>, tonic::Status> {
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
                "/google.shopping.merchant.inventories.v1beta.LocalInventoryService/InsertLocalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.LocalInventoryService",
                        "InsertLocalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified `LocalInventory` from the given product in your
        /// merchant account. It might take a up to an hour for the
        /// `LocalInventory` to be deleted from the specific product.
        /// Once you have received a successful delete response, wait for that
        /// period before attempting a delete again.
        pub async fn delete_local_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLocalInventoryRequest>,
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
                "/google.shopping.merchant.inventories.v1beta.LocalInventoryService/DeleteLocalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.LocalInventoryService",
                        "DeleteLocalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionalInventory {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub account: i64,
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    #[prost(message, optional, tag = "5")]
    pub sale_price: ::core::option::Option<super::super::super::r#type::Price>,
    #[prost(message, optional, tag = "6")]
    pub sale_price_effective_date: ::core::option::Option<
        super::super::super::super::r#type::Interval,
    >,
    #[prost(string, optional, tag = "7")]
    pub availability: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub custom_attributes: ::prost::alloc::vec::Vec<
        super::super::super::r#type::CustomAttribute,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegionalInventoriesRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegionalInventoriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub regional_inventories: ::prost::alloc::vec::Vec<RegionalInventory>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertRegionalInventoryRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub regional_inventory: ::core::option::Option<RegionalInventory>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRegionalInventoryRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod regional_inventory_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage regional inventory for products. There is also separate
    /// `regions` resource and API to manage regions definitions.
    #[derive(Debug, Clone)]
    pub struct RegionalInventoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RegionalInventoryServiceClient<T>
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
        ) -> RegionalInventoryServiceClient<InterceptedService<T, F>>
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
            RegionalInventoryServiceClient::new(
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
        /// Lists the `RegionalInventory` resources for the given product in your
        /// merchant account. The response might contain fewer items than specified by
        /// `pageSize`.  If `pageToken` was returned in previous request, it can be
        /// used to obtain additional results.
        ///
        /// `RegionalInventory` resources are listed per product for a given account.
        pub async fn list_regional_inventories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRegionalInventoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRegionalInventoriesResponse>,
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
                "/google.shopping.merchant.inventories.v1beta.RegionalInventoryService/ListRegionalInventories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.RegionalInventoryService",
                        "ListRegionalInventories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Inserts a `RegionalInventory` to a given product in your
        /// merchant account.
        ///
        /// Replaces the full `RegionalInventory` resource if an entry with the same
        /// \[`region`\]\[google.shopping.merchant.inventories.v1beta.RegionalInventory.region\]
        /// already exists for the product.
        ///
        /// It might take up to 30 minutes for the new or updated `RegionalInventory`
        /// resource to appear in products.
        pub async fn insert_regional_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertRegionalInventoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegionalInventory>,
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
                "/google.shopping.merchant.inventories.v1beta.RegionalInventoryService/InsertRegionalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.RegionalInventoryService",
                        "InsertRegionalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified `RegionalInventory` resource from the given product
        /// in your merchant account.  It might take up to an hour for the
        /// `RegionalInventory` to be deleted from the specific product.
        /// Once you have received a successful delete response, wait for that
        /// period before attempting a delete again.
        pub async fn delete_regional_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRegionalInventoryRequest>,
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
                "/google.shopping.merchant.inventories.v1beta.RegionalInventoryService/DeleteRegionalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.RegionalInventoryService",
                        "DeleteRegionalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
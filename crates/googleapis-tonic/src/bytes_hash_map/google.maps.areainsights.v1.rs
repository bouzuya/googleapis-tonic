// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeInsightsRequest {
    #[prost(enumeration = "Insight", repeated, packed = "false", tag = "4")]
    pub insights: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "5")]
    pub filter: ::core::option::Option<Filter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeInsightsResponse {
    #[prost(int64, optional, tag = "1")]
    pub count: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "5")]
    pub place_insights: ::prost::alloc::vec::Vec<PlaceInsight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceInsight {
    #[prost(string, tag = "1")]
    pub place: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(message, optional, tag = "1")]
    pub location_filter: ::core::option::Option<LocationFilter>,
    #[prost(message, optional, tag = "2")]
    pub type_filter: ::core::option::Option<TypeFilter>,
    #[prost(enumeration = "OperatingStatus", repeated, packed = "false", tag = "3")]
    pub operating_status: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "PriceLevel", repeated, packed = "false", tag = "4")]
    pub price_levels: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "5")]
    pub rating_filter: ::core::option::Option<RatingFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationFilter {
    #[prost(oneof = "location_filter::Area", tags = "1, 2, 3")]
    pub area: ::core::option::Option<location_filter::Area>,
}
/// Nested message and enum types in `LocationFilter`.
pub mod location_filter {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Circle {
        #[prost(int32, tag = "3")]
        pub radius: i32,
        #[prost(oneof = "circle::Center", tags = "1, 2")]
        pub center: ::core::option::Option<circle::Center>,
    }
    /// Nested message and enum types in `Circle`.
    pub mod circle {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Center {
            #[prost(message, tag = "1")]
            LatLng(super::super::super::super::super::r#type::LatLng),
            #[prost(string, tag = "2")]
            Place(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Region {
        #[prost(oneof = "region::Region", tags = "1")]
        pub region: ::core::option::Option<region::Region>,
    }
    /// Nested message and enum types in `Region`.
    pub mod region {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Region {
            #[prost(string, tag = "1")]
            Place(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomArea {
        #[prost(message, optional, tag = "1")]
        pub polygon: ::core::option::Option<custom_area::Polygon>,
    }
    /// Nested message and enum types in `CustomArea`.
    pub mod custom_area {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Polygon {
            #[prost(message, repeated, tag = "1")]
            pub coordinates: ::prost::alloc::vec::Vec<
                super::super::super::super::super::r#type::LatLng,
            >,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Area {
        #[prost(message, tag = "1")]
        Circle(Circle),
        #[prost(message, tag = "2")]
        Region(Region),
        #[prost(message, tag = "3")]
        CustomArea(CustomArea),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeFilter {
    #[prost(string, repeated, tag = "1")]
    pub included_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub excluded_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub included_primary_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub excluded_primary_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RatingFilter {
    #[prost(float, optional, tag = "5")]
    pub min_rating: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "6")]
    pub max_rating: ::core::option::Option<f32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Insight {
    Unspecified = 0,
    Count = 1,
    Places = 2,
}
impl Insight {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Insight::Unspecified => "INSIGHT_UNSPECIFIED",
            Insight::Count => "INSIGHT_COUNT",
            Insight::Places => "INSIGHT_PLACES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSIGHT_UNSPECIFIED" => Some(Self::Unspecified),
            "INSIGHT_COUNT" => Some(Self::Count),
            "INSIGHT_PLACES" => Some(Self::Places),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperatingStatus {
    Unspecified = 0,
    Operational = 1,
    PermanentlyClosed = 3,
    TemporarilyClosed = 4,
}
impl OperatingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperatingStatus::Unspecified => "OPERATING_STATUS_UNSPECIFIED",
            OperatingStatus::Operational => "OPERATING_STATUS_OPERATIONAL",
            OperatingStatus::PermanentlyClosed => "OPERATING_STATUS_PERMANENTLY_CLOSED",
            OperatingStatus::TemporarilyClosed => "OPERATING_STATUS_TEMPORARILY_CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATING_STATUS_OPERATIONAL" => Some(Self::Operational),
            "OPERATING_STATUS_PERMANENTLY_CLOSED" => Some(Self::PermanentlyClosed),
            "OPERATING_STATUS_TEMPORARILY_CLOSED" => Some(Self::TemporarilyClosed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceLevel {
    Unspecified = 0,
    Free = 1,
    Inexpensive = 2,
    Moderate = 3,
    Expensive = 4,
    VeryExpensive = 5,
}
impl PriceLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceLevel::Unspecified => "PRICE_LEVEL_UNSPECIFIED",
            PriceLevel::Free => "PRICE_LEVEL_FREE",
            PriceLevel::Inexpensive => "PRICE_LEVEL_INEXPENSIVE",
            PriceLevel::Moderate => "PRICE_LEVEL_MODERATE",
            PriceLevel::Expensive => "PRICE_LEVEL_EXPENSIVE",
            PriceLevel::VeryExpensive => "PRICE_LEVEL_VERY_EXPENSIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_LEVEL_FREE" => Some(Self::Free),
            "PRICE_LEVEL_INEXPENSIVE" => Some(Self::Inexpensive),
            "PRICE_LEVEL_MODERATE" => Some(Self::Moderate),
            "PRICE_LEVEL_EXPENSIVE" => Some(Self::Expensive),
            "PRICE_LEVEL_VERY_EXPENSIVE" => Some(Self::VeryExpensive),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod area_insights_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Area Insights API.
    #[derive(Debug, Clone)]
    pub struct AreaInsightsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AreaInsightsClient<T>
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
        ) -> AreaInsightsClient<InterceptedService<T, F>>
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
            AreaInsightsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Compute Insights RPC
        ///
        /// This method lets you retrieve insights about areas using a variaty of
        /// filter such as: area, place type, operating status, price level
        /// and ratings. Currently "count" and "places" insights are supported. With
        /// "count" insights you can answer questions such as "How many restaurant are
        /// located in California that are operational, are inexpensive and have an
        /// average rating of at least 4 stars" (see `insight` enum for more details).
        /// With "places" insights, you can determine which places match the
        /// requested filter. Clients can then use those place resource names to fetch
        /// more details about each individual place using the Places API.
        pub async fn compute_insights(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeInsightsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ComputeInsightsResponse>,
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
                "/google.maps.areainsights.v1.AreaInsights/ComputeInsights",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.areainsights.v1.AreaInsights",
                        "ComputeInsights",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
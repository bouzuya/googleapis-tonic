// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TextSentimentPredictionResult {
    #[prost(int32, tag = "1")]
    pub sentiment: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionResult {
    #[prost(string, tag = "1")]
    pub category_mask: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub confidence_mask: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "5")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "6")]
    pub confidence: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TimeSeriesForecastingPredictionResult {
    #[prost(float, tag = "1")]
    pub value: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "4")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "5")]
    pub confidence: ::core::option::Option<f32>,
    #[prost(message, repeated, tag = "6")]
    pub frames: ::prost::alloc::vec::Vec<video_object_tracking_prediction_result::Frame>,
}
/// Nested message and enum types in `VideoObjectTrackingPredictionResult`.
pub mod video_object_tracking_prediction_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Frame {
        #[prost(message, optional, tag = "1")]
        pub time_offset: ::core::option::Option<::prost_types::Duration>,
        #[prost(message, optional, tag = "2")]
        pub x_min: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "3")]
        pub x_max: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "4")]
        pub y_min: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "5")]
        pub y_max: ::core::option::Option<f32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "5")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "6")]
    pub confidence: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionResult {
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
    #[prost(message, repeated, tag = "4")]
    pub bboxes: ::prost::alloc::vec::Vec<::prost_types::ListValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TabularRegressionPredictionResult {
    #[prost(float, tag = "1")]
    pub value: f32,
    #[prost(float, tag = "2")]
    pub lower_bound: f32,
    #[prost(float, tag = "3")]
    pub upper_bound: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabularClassificationPredictionResult {
    #[prost(string, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(float, repeated, tag = "2")]
    pub scores: ::prost::alloc::vec::Vec<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationPredictionResult {
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionPredictionResult {
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "3")]
    pub text_segment_start_offsets: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "4")]
    pub text_segment_end_offsets: ::prost::alloc::vec::Vec<i64>,
    #[prost(float, repeated, tag = "5")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
}
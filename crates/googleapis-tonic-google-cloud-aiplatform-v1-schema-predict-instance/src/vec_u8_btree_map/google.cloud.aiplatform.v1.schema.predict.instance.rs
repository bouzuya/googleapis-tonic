// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationPredictionInstance {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
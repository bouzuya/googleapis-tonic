// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVoicesRequest {
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVoicesResponse {
    #[prost(message, repeated, tag = "1")]
    pub voices: ::prost::alloc::vec::Vec<Voice>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Voice {
    #[prost(string, repeated, tag = "1")]
    pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "SsmlVoiceGender", tag = "3")]
    pub ssml_gender: i32,
    #[prost(int32, tag = "4")]
    pub natural_sample_rate_hertz: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechRequest {
    #[prost(message, optional, tag = "1")]
    pub input: ::core::option::Option<SynthesisInput>,
    #[prost(message, optional, tag = "2")]
    pub voice: ::core::option::Option<VoiceSelectionParams>,
    #[prost(message, optional, tag = "3")]
    pub audio_config: ::core::option::Option<AudioConfig>,
    #[prost(
        enumeration = "synthesize_speech_request::TimepointType",
        repeated,
        tag = "4"
    )]
    pub enable_time_pointing: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `SynthesizeSpeechRequest`.
pub mod synthesize_speech_request {
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
    pub enum TimepointType {
        Unspecified = 0,
        SsmlMark = 1,
    }
    impl TimepointType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimepointType::Unspecified => "TIMEPOINT_TYPE_UNSPECIFIED",
                TimepointType::SsmlMark => "SSML_MARK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIMEPOINT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SSML_MARK" => Some(Self::SsmlMark),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesisInput {
    #[prost(oneof = "synthesis_input::InputSource", tags = "1, 2")]
    pub input_source: ::core::option::Option<synthesis_input::InputSource>,
}
/// Nested message and enum types in `SynthesisInput`.
pub mod synthesis_input {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InputSource {
        #[prost(string, tag = "1")]
        Text(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Ssml(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceSelectionParams {
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "SsmlVoiceGender", tag = "3")]
    pub ssml_gender: i32,
    #[prost(message, optional, tag = "4")]
    pub custom_voice: ::core::option::Option<CustomVoiceParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioConfig {
    #[prost(enumeration = "AudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    #[prost(double, tag = "2")]
    pub speaking_rate: f64,
    #[prost(double, tag = "3")]
    pub pitch: f64,
    #[prost(double, tag = "4")]
    pub volume_gain_db: f64,
    #[prost(int32, tag = "5")]
    pub sample_rate_hertz: i32,
    #[prost(string, repeated, tag = "6")]
    pub effects_profile_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomVoiceParams {
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(enumeration = "custom_voice_params::ReportedUsage", tag = "3")]
    pub reported_usage: i32,
}
/// Nested message and enum types in `CustomVoiceParams`.
pub mod custom_voice_params {
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
    pub enum ReportedUsage {
        Unspecified = 0,
        Realtime = 1,
        Offline = 2,
    }
    impl ReportedUsage {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReportedUsage::Unspecified => "REPORTED_USAGE_UNSPECIFIED",
                ReportedUsage::Realtime => "REALTIME",
                ReportedUsage::Offline => "OFFLINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REPORTED_USAGE_UNSPECIFIED" => Some(Self::Unspecified),
                "REALTIME" => Some(Self::Realtime),
                "OFFLINE" => Some(Self::Offline),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechResponse {
    #[prost(bytes = "bytes", tag = "1")]
    pub audio_content: ::prost::bytes::Bytes,
    #[prost(message, repeated, tag = "2")]
    pub timepoints: ::prost::alloc::vec::Vec<Timepoint>,
    #[prost(message, optional, tag = "4")]
    pub audio_config: ::core::option::Option<AudioConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timepoint {
    #[prost(string, tag = "4")]
    pub mark_name: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub time_seconds: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SsmlVoiceGender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
    Neutral = 3,
}
impl SsmlVoiceGender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SsmlVoiceGender::Unspecified => "SSML_VOICE_GENDER_UNSPECIFIED",
            SsmlVoiceGender::Male => "MALE",
            SsmlVoiceGender::Female => "FEMALE",
            SsmlVoiceGender::Neutral => "NEUTRAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SSML_VOICE_GENDER_UNSPECIFIED" => Some(Self::Unspecified),
            "MALE" => Some(Self::Male),
            "FEMALE" => Some(Self::Female),
            "NEUTRAL" => Some(Self::Neutral),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioEncoding {
    Unspecified = 0,
    Linear16 = 1,
    Mp3 = 2,
    Mp364Kbps = 4,
    OggOpus = 3,
    Mulaw = 5,
    Alaw = 6,
}
impl AudioEncoding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AudioEncoding::Unspecified => "AUDIO_ENCODING_UNSPECIFIED",
            AudioEncoding::Linear16 => "LINEAR16",
            AudioEncoding::Mp3 => "MP3",
            AudioEncoding::Mp364Kbps => "MP3_64_KBPS",
            AudioEncoding::OggOpus => "OGG_OPUS",
            AudioEncoding::Mulaw => "MULAW",
            AudioEncoding::Alaw => "ALAW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUDIO_ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
            "LINEAR16" => Some(Self::Linear16),
            "MP3" => Some(Self::Mp3),
            "MP3_64_KBPS" => Some(Self::Mp364Kbps),
            "OGG_OPUS" => Some(Self::OggOpus),
            "MULAW" => Some(Self::Mulaw),
            "ALAW" => Some(Self::Alaw),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod text_to_speech_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that implements Google Cloud Text-to-Speech API.
    #[derive(Debug, Clone)]
    pub struct TextToSpeechClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TextToSpeechClient<T>
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
        ) -> TextToSpeechClient<InterceptedService<T, F>>
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
            TextToSpeechClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns a list of Voice supported for synthesis.
        pub async fn list_voices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVoicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVoicesResponse>,
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
                "/google.cloud.texttospeech.v1beta1.TextToSpeech/ListVoices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.texttospeech.v1beta1.TextToSpeech",
                        "ListVoices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Synthesizes speech synchronously: receive results after all text input
        /// has been processed.
        pub async fn synthesize_speech(
            &mut self,
            request: impl tonic::IntoRequest<super::SynthesizeSpeechRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SynthesizeSpeechResponse>,
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
                "/google.cloud.texttospeech.v1beta1.TextToSpeech/SynthesizeSpeech",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.texttospeech.v1beta1.TextToSpeech",
                        "SynthesizeSpeech",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeLongAudioRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<SynthesisInput>,
    #[prost(message, optional, tag = "3")]
    pub audio_config: ::core::option::Option<AudioConfig>,
    #[prost(string, tag = "4")]
    pub output_gcs_uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub voice: ::core::option::Option<VoiceSelectionParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SynthesizeLongAudioResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SynthesizeLongAudioMetadata {
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(double, tag = "3")]
    pub progress_percentage: f64,
}
/// Generated client implementations.
pub mod text_to_speech_long_audio_synthesize_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that implements Google Cloud Text-to-Speech API.
    #[derive(Debug, Clone)]
    pub struct TextToSpeechLongAudioSynthesizeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TextToSpeechLongAudioSynthesizeClient<T>
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
        ) -> TextToSpeechLongAudioSynthesizeClient<InterceptedService<T, F>>
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
            TextToSpeechLongAudioSynthesizeClient::new(
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
        /// Synthesizes long form text asynchronously.
        pub async fn synthesize_long_audio(
            &mut self,
            request: impl tonic::IntoRequest<super::SynthesizeLongAudioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.texttospeech.v1beta1.TextToSpeechLongAudioSynthesize/SynthesizeLongAudio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.texttospeech.v1beta1.TextToSpeechLongAudioSynthesize",
                        "SynthesizeLongAudio",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
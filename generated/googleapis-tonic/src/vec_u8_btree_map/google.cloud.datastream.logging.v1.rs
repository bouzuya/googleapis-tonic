// This file is @generated by prost-build.
/// Log definition for activities related to a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamActivityLogEntry {
    /// A code representing the event.
    #[prost(string, tag = "1")]
    pub event_code: ::prost::alloc::string::String,
    /// A free-text message describing the event.
    #[prost(string, tag = "2")]
    pub event_message: ::prost::alloc::string::String,
    #[prost(oneof = "stream_activity_log_entry::EventPayload", tags = "100")]
    pub event_payload: ::core::option::Option<stream_activity_log_entry::EventPayload>,
}
/// Nested message and enum types in `StreamActivityLogEntry`.
pub mod stream_activity_log_entry {
    /// Payload for a change in the state of a stream.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct StreamStateChange {
        /// Output only. The new stream state.
        #[prost(enumeration = "super::super::super::v1::stream::State", tag = "1")]
        pub new_state: i32,
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum EventPayload {
        /// A payload for a change in the state of a stream.
        #[prost(message, tag = "100")]
        StreamStateChange(StreamStateChange),
    }
}

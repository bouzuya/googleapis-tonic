// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    #[prost(string, tag = "1")]
    pub principal: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub permission: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedPolicy {
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access: i32,
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    #[prost(message, repeated, tag = "4")]
    pub binding_explanations: ::prost::alloc::vec::Vec<BindingExplanation>,
    #[prost(enumeration = "HeuristicRelevance", tag = "5")]
    pub relevance: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindingExplanation {
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access: i32,
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
    #[prost(enumeration = "binding_explanation::RolePermission", tag = "3")]
    pub role_permission: i32,
    #[prost(enumeration = "HeuristicRelevance", tag = "4")]
    pub role_permission_relevance: i32,
    #[prost(map = "string, message", tag = "5")]
    pub memberships: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        binding_explanation::AnnotatedMembership,
    >,
    #[prost(enumeration = "HeuristicRelevance", tag = "6")]
    pub relevance: i32,
    #[prost(message, optional, tag = "7")]
    pub condition: ::core::option::Option<super::super::super::r#type::Expr>,
}
/// Nested message and enum types in `BindingExplanation`.
pub mod binding_explanation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct AnnotatedMembership {
        #[prost(enumeration = "Membership", tag = "1")]
        pub membership: i32,
        #[prost(enumeration = "super::HeuristicRelevance", tag = "2")]
        pub relevance: i32,
    }
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
    pub enum RolePermission {
        Unspecified = 0,
        Included = 1,
        NotIncluded = 2,
        UnknownInfoDenied = 3,
    }
    impl RolePermission {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolePermission::Unspecified => "ROLE_PERMISSION_UNSPECIFIED",
                RolePermission::Included => "ROLE_PERMISSION_INCLUDED",
                RolePermission::NotIncluded => "ROLE_PERMISSION_NOT_INCLUDED",
                RolePermission::UnknownInfoDenied => {
                    "ROLE_PERMISSION_UNKNOWN_INFO_DENIED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLE_PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
                "ROLE_PERMISSION_INCLUDED" => Some(Self::Included),
                "ROLE_PERMISSION_NOT_INCLUDED" => Some(Self::NotIncluded),
                "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => Some(Self::UnknownInfoDenied),
                _ => None,
            }
        }
    }
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
    pub enum Membership {
        Unspecified = 0,
        Included = 1,
        NotIncluded = 2,
        UnknownInfoDenied = 3,
        UnknownUnsupported = 4,
    }
    impl Membership {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Membership::Unspecified => "MEMBERSHIP_UNSPECIFIED",
                Membership::Included => "MEMBERSHIP_INCLUDED",
                Membership::NotIncluded => "MEMBERSHIP_NOT_INCLUDED",
                Membership::UnknownInfoDenied => "MEMBERSHIP_UNKNOWN_INFO_DENIED",
                Membership::UnknownUnsupported => "MEMBERSHIP_UNKNOWN_UNSUPPORTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MEMBERSHIP_UNSPECIFIED" => Some(Self::Unspecified),
                "MEMBERSHIP_INCLUDED" => Some(Self::Included),
                "MEMBERSHIP_NOT_INCLUDED" => Some(Self::NotIncluded),
                "MEMBERSHIP_UNKNOWN_INFO_DENIED" => Some(Self::UnknownInfoDenied),
                "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => Some(Self::UnknownUnsupported),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessState {
    Unspecified = 0,
    Granted = 1,
    NotGranted = 2,
    UnknownConditional = 3,
    UnknownInfoDenied = 4,
}
impl AccessState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessState::Unspecified => "ACCESS_STATE_UNSPECIFIED",
            AccessState::Granted => "GRANTED",
            AccessState::NotGranted => "NOT_GRANTED",
            AccessState::UnknownConditional => "UNKNOWN_CONDITIONAL",
            AccessState::UnknownInfoDenied => "UNKNOWN_INFO_DENIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "GRANTED" => Some(Self::Granted),
            "NOT_GRANTED" => Some(Self::NotGranted),
            "UNKNOWN_CONDITIONAL" => Some(Self::UnknownConditional),
            "UNKNOWN_INFO_DENIED" => Some(Self::UnknownInfoDenied),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HeuristicRelevance {
    Unspecified = 0,
    Normal = 1,
    High = 2,
}
impl HeuristicRelevance {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HeuristicRelevance::Unspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            HeuristicRelevance::Normal => "NORMAL",
            HeuristicRelevance::High => "HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEURISTIC_RELEVANCE_UNSPECIFIED" => Some(Self::Unspecified),
            "NORMAL" => Some(Self::Normal),
            "HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replay {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "replay::State", tag = "2")]
    pub state: i32,
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<ReplayConfig>,
    #[prost(message, optional, tag = "5")]
    pub results_summary: ::core::option::Option<replay::ResultsSummary>,
}
/// Nested message and enum types in `Replay`.
pub mod replay {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ResultsSummary {
        #[prost(int32, tag = "1")]
        pub log_count: i32,
        #[prost(int32, tag = "2")]
        pub unchanged_count: i32,
        #[prost(int32, tag = "3")]
        pub difference_count: i32,
        #[prost(int32, tag = "4")]
        pub error_count: i32,
        #[prost(message, optional, tag = "5")]
        pub oldest_date: ::core::option::Option<
            super::super::super::super::r#type::Date,
        >,
        #[prost(message, optional, tag = "6")]
        pub newest_date: ::core::option::Option<
            super::super::super::super::r#type::Date,
        >,
    }
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
    pub enum State {
        Unspecified = 0,
        Pending = 1,
        Running = 2,
        Succeeded = 3,
        Failed = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayResult {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub access_tuple: ::core::option::Option<AccessTuple>,
    #[prost(message, optional, tag = "4")]
    pub last_seen_date: ::core::option::Option<super::super::super::r#type::Date>,
    #[prost(oneof = "replay_result::Result", tags = "5, 6")]
    pub result: ::core::option::Option<replay_result::Result>,
}
/// Nested message and enum types in `ReplayResult`.
pub mod replay_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "5")]
        Diff(super::ReplayDiff),
        #[prost(message, tag = "6")]
        Error(super::super::super::super::rpc::Status),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReplayRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub replay: ::core::option::Option<Replay>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ReplayOperationMetadata {
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReplayRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplayResultsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplayResultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub replay_results: ::prost::alloc::vec::Vec<ReplayResult>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayConfig {
    #[prost(map = "string, message", tag = "1")]
    pub policy_overlay: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::iam::v1::Policy,
    >,
    #[prost(enumeration = "replay_config::LogSource", tag = "2")]
    pub log_source: i32,
}
/// Nested message and enum types in `ReplayConfig`.
pub mod replay_config {
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
    pub enum LogSource {
        Unspecified = 0,
        RecentAccesses = 1,
    }
    impl LogSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LogSource::Unspecified => "LOG_SOURCE_UNSPECIFIED",
                LogSource::RecentAccesses => "RECENT_ACCESSES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOG_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
                "RECENT_ACCESSES" => Some(Self::RecentAccesses),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayDiff {
    #[prost(message, optional, tag = "2")]
    pub access_diff: ::core::option::Option<AccessStateDiff>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessStateDiff {
    #[prost(message, optional, tag = "1")]
    pub baseline: ::core::option::Option<ExplainedAccess>,
    #[prost(message, optional, tag = "2")]
    pub simulated: ::core::option::Option<ExplainedAccess>,
    #[prost(enumeration = "access_state_diff::AccessChangeType", tag = "3")]
    pub access_change: i32,
}
/// Nested message and enum types in `AccessStateDiff`.
pub mod access_state_diff {
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
    pub enum AccessChangeType {
        Unspecified = 0,
        NoChange = 1,
        UnknownChange = 2,
        AccessRevoked = 3,
        AccessGained = 4,
        AccessMaybeRevoked = 5,
        AccessMaybeGained = 6,
    }
    impl AccessChangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccessChangeType::Unspecified => "ACCESS_CHANGE_TYPE_UNSPECIFIED",
                AccessChangeType::NoChange => "NO_CHANGE",
                AccessChangeType::UnknownChange => "UNKNOWN_CHANGE",
                AccessChangeType::AccessRevoked => "ACCESS_REVOKED",
                AccessChangeType::AccessGained => "ACCESS_GAINED",
                AccessChangeType::AccessMaybeRevoked => "ACCESS_MAYBE_REVOKED",
                AccessChangeType::AccessMaybeGained => "ACCESS_MAYBE_GAINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCESS_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NO_CHANGE" => Some(Self::NoChange),
                "UNKNOWN_CHANGE" => Some(Self::UnknownChange),
                "ACCESS_REVOKED" => Some(Self::AccessRevoked),
                "ACCESS_GAINED" => Some(Self::AccessGained),
                "ACCESS_MAYBE_REVOKED" => Some(Self::AccessMaybeRevoked),
                "ACCESS_MAYBE_GAINED" => Some(Self::AccessMaybeGained),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedAccess {
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access_state: i32,
    #[prost(message, repeated, tag = "2")]
    pub policies: ::prost::alloc::vec::Vec<ExplainedPolicy>,
    #[prost(message, repeated, tag = "3")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Generated client implementations.
pub mod simulator_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Policy Simulator API service.
    ///
    /// Policy Simulator is a collection of endpoints for creating, running, and
    /// viewing a \[Replay\]\[google.cloud.policysimulator.v1.Replay\]. A
    /// \[Replay\]\[google.cloud.policysimulator.v1.Replay\] is a type of simulation that
    /// lets you see how your principals' access to resources might change if you
    /// changed your IAM policy.
    ///
    /// During a \[Replay\]\[google.cloud.policysimulator.v1.Replay\], Policy Simulator
    /// re-evaluates, or replays, past access attempts under both the current policy
    /// and  your proposed policy, and compares those results to determine how your
    /// principals' access might change under the proposed policy.
    #[derive(Debug, Clone)]
    pub struct SimulatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SimulatorClient<T>
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
        ) -> SimulatorClient<InterceptedService<T, F>>
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
            SimulatorClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets the specified \[Replay\]\[google.cloud.policysimulator.v1.Replay\]. Each
        /// `Replay` is available for at least 7 days.
        pub async fn get_replay(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReplayRequest>,
        ) -> std::result::Result<tonic::Response<super::Replay>, tonic::Status> {
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
                "/google.cloud.policysimulator.v1.Simulator/GetReplay",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.policysimulator.v1.Simulator",
                        "GetReplay",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates and starts a \[Replay\]\[google.cloud.policysimulator.v1.Replay\] using
        /// the given \[ReplayConfig\]\[google.cloud.policysimulator.v1.ReplayConfig\].
        pub async fn create_replay(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReplayRequest>,
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
                "/google.cloud.policysimulator.v1.Simulator/CreateReplay",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.policysimulator.v1.Simulator",
                        "CreateReplay",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the results of running a
        /// \[Replay\]\[google.cloud.policysimulator.v1.Replay\].
        pub async fn list_replay_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReplayResultsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReplayResultsResponse>,
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
                "/google.cloud.policysimulator.v1.Simulator/ListReplayResults",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.policysimulator.v1.Simulator",
                        "ListReplayResults",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
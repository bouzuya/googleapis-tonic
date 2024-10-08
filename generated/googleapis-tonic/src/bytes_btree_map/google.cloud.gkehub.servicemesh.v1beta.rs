// This file is @generated by prost-build.
/// **Service Mesh**: Spec for a single Membership for the servicemesh feature
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MembershipSpec {
    /// Deprecated: use `management` instead
    /// Enables automatic control plane management.
    #[deprecated]
    #[prost(enumeration = "membership_spec::ControlPlaneManagement", tag = "1")]
    pub control_plane: i32,
    /// Enables automatic Service Mesh management.
    #[prost(enumeration = "membership_spec::Management", tag = "4")]
    pub management: i32,
}
/// Nested message and enum types in `MembershipSpec`.
pub mod membership_spec {
    /// Whether to automatically manage Service Mesh control planes.
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
    pub enum ControlPlaneManagement {
        /// Unspecified
        Unspecified = 0,
        /// Google should provision a control plane revision and make it available in
        /// the cluster. Google will enroll this revision in a release channel and
        /// keep it up to date. The control plane revision may be a managed service,
        /// or a managed install.
        Automatic = 1,
        /// User will manually configure the control plane (e.g. via CLI, or via the
        /// ControlPlaneRevision KRM API)
        Manual = 2,
    }
    impl ControlPlaneManagement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "CONTROL_PLANE_MANAGEMENT_UNSPECIFIED",
                Self::Automatic => "AUTOMATIC",
                Self::Manual => "MANUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTROL_PLANE_MANAGEMENT_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOMATIC" => Some(Self::Automatic),
                "MANUAL" => Some(Self::Manual),
                _ => None,
            }
        }
    }
    /// Whether to automatically manage Service Mesh.
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
    pub enum Management {
        /// Unspecified
        Unspecified = 0,
        /// Google should manage my Service Mesh for the cluster.
        Automatic = 1,
        /// User will manually configure their service mesh components.
        Manual = 2,
    }
    impl Management {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "MANAGEMENT_UNSPECIFIED",
                Self::Automatic => "MANAGEMENT_AUTOMATIC",
                Self::Manual => "MANAGEMENT_MANUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANAGEMENT_UNSPECIFIED" => Some(Self::Unspecified),
                "MANAGEMENT_AUTOMATIC" => Some(Self::Automatic),
                "MANAGEMENT_MANUAL" => Some(Self::Manual),
                _ => None,
            }
        }
    }
}
/// **Service Mesh**: State for a single Membership, as analyzed by the Service
/// Mesh Hub Controller.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipState {
    /// Output only. Status of control plane management
    #[prost(message, optional, tag = "2")]
    pub control_plane_management: ::core::option::Option<
        membership_state::ControlPlaneManagement,
    >,
    /// Output only. Status of data plane management.
    #[prost(message, optional, tag = "4")]
    pub data_plane_management: ::core::option::Option<
        membership_state::DataPlaneManagement,
    >,
    /// Output only. List of conditions reported for this membership.
    #[prost(message, repeated, tag = "8")]
    pub conditions: ::prost::alloc::vec::Vec<membership_state::Condition>,
}
/// Nested message and enum types in `MembershipState`.
pub mod membership_state {
    /// Status of control plane management.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ControlPlaneManagement {
        /// Explanation of state.
        #[prost(message, repeated, tag = "2")]
        pub details: ::prost::alloc::vec::Vec<super::StatusDetails>,
        /// LifecycleState of control plane management.
        #[prost(enumeration = "LifecycleState", tag = "3")]
        pub state: i32,
        /// Output only. Implementation of managed control plane.
        #[prost(enumeration = "control_plane_management::Implementation", tag = "4")]
        pub implementation: i32,
    }
    /// Nested message and enum types in `ControlPlaneManagement`.
    pub mod control_plane_management {
        /// Implementation of managed control plane.
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
        pub enum Implementation {
            /// Unspecified
            Unspecified = 0,
            /// A Google build of istiod is used for the managed control plane.
            Istiod = 1,
            /// Traffic director is used for the managed control plane.
            TrafficDirector = 2,
            /// The control plane implementation is being updated.
            Updating = 3,
        }
        impl Implementation {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "IMPLEMENTATION_UNSPECIFIED",
                    Self::Istiod => "ISTIOD",
                    Self::TrafficDirector => "TRAFFIC_DIRECTOR",
                    Self::Updating => "UPDATING",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "IMPLEMENTATION_UNSPECIFIED" => Some(Self::Unspecified),
                    "ISTIOD" => Some(Self::Istiod),
                    "TRAFFIC_DIRECTOR" => Some(Self::TrafficDirector),
                    "UPDATING" => Some(Self::Updating),
                    _ => None,
                }
            }
        }
    }
    /// Status of data plane management. Only reported per-member.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataPlaneManagement {
        /// Lifecycle status of data plane management.
        #[prost(enumeration = "LifecycleState", tag = "1")]
        pub state: i32,
        /// Explanation of the status.
        #[prost(message, repeated, tag = "2")]
        pub details: ::prost::alloc::vec::Vec<super::StatusDetails>,
    }
    /// Condition being reported.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Condition {
        /// Unique identifier of the condition which describes the condition
        /// recognizable to the user.
        #[prost(enumeration = "condition::Code", tag = "1")]
        pub code: i32,
        /// Links contains actionable information.
        #[prost(string, tag = "2")]
        pub documentation_link: ::prost::alloc::string::String,
        /// A short summary about the issue.
        #[prost(string, tag = "3")]
        pub details: ::prost::alloc::string::String,
        /// Severity level of the condition.
        #[prost(enumeration = "condition::Severity", tag = "4")]
        pub severity: i32,
    }
    /// Nested message and enum types in `Condition`.
    pub mod condition {
        /// Unique identifier of the condition which describes the condition
        /// recognizable to the user.
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
        pub enum Code {
            /// Default Unspecified code
            Unspecified = 0,
            /// Mesh IAM permission denied error code
            MeshIamPermissionDenied = 100,
            /// CNI config unsupported error code
            CniConfigUnsupported = 201,
            /// GKE sandbox unsupported error code
            GkeSandboxUnsupported = 202,
            /// Nodepool workload identity federation required error code
            NodepoolWorkloadIdentityFederationRequired = 203,
            /// CNI installation failed error code
            CniInstallationFailed = 204,
            /// CNI pod unschedulable error code
            CniPodUnschedulable = 205,
            /// Multiple control planes unsupported error code
            UnsupportedMultipleControlPlanes = 301,
            /// VPC-SC GA is supported for this control plane.
            VpcscGaSupported = 302,
            /// Configuration (Istio/k8s resources) failed to apply due to internal
            /// error.
            ConfigApplyInternalError = 401,
            /// Configuration failed to be applied due to being invalid.
            ConfigValidationError = 402,
            /// Encountered configuration(s) with possible unintended behavior or
            /// invalid configuration. These configs may not have been applied.
            ConfigValidationWarning = 403,
            /// BackendService quota exceeded error code.
            QuotaExceededBackendServices = 404,
            /// HealthCheck quota exceeded error code.
            QuotaExceededHealthChecks = 405,
            /// HTTPRoute quota exceeded error code.
            QuotaExceededHttpRoutes = 406,
            /// TCPRoute quota exceeded error code.
            QuotaExceededTcpRoutes = 407,
            /// TLS routes quota exceeded error code.
            QuotaExceededTlsRoutes = 408,
            /// TrafficPolicy quota exceeded error code.
            QuotaExceededTrafficPolicies = 409,
            /// EndpointPolicy quota exceeded error code.
            QuotaExceededEndpointPolicies = 410,
            /// Gateway quota exceeded error code.
            QuotaExceededGateways = 411,
            /// Mesh quota exceeded error code.
            QuotaExceededMeshes = 412,
            /// ServerTLSPolicy quota exceeded error code.
            QuotaExceededServerTlsPolicies = 413,
            /// ClientTLSPolicy quota exceeded error code.
            QuotaExceededClientTlsPolicies = 414,
            /// ServiceLBPolicy quota exceeded error code.
            QuotaExceededServiceLbPolicies = 415,
            /// HTTPFilter quota exceeded error code.
            QuotaExceededHttpFilters = 416,
            /// TCPFilter quota exceeded error code.
            QuotaExceededTcpFilters = 417,
            /// NetworkEndpointGroup quota exceeded error code.
            QuotaExceededNetworkEndpointGroups = 418,
        }
        impl Code {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "CODE_UNSPECIFIED",
                    Self::MeshIamPermissionDenied => "MESH_IAM_PERMISSION_DENIED",
                    Self::CniConfigUnsupported => "CNI_CONFIG_UNSUPPORTED",
                    Self::GkeSandboxUnsupported => "GKE_SANDBOX_UNSUPPORTED",
                    Self::NodepoolWorkloadIdentityFederationRequired => {
                        "NODEPOOL_WORKLOAD_IDENTITY_FEDERATION_REQUIRED"
                    }
                    Self::CniInstallationFailed => "CNI_INSTALLATION_FAILED",
                    Self::CniPodUnschedulable => "CNI_POD_UNSCHEDULABLE",
                    Self::UnsupportedMultipleControlPlanes => {
                        "UNSUPPORTED_MULTIPLE_CONTROL_PLANES"
                    }
                    Self::VpcscGaSupported => "VPCSC_GA_SUPPORTED",
                    Self::ConfigApplyInternalError => "CONFIG_APPLY_INTERNAL_ERROR",
                    Self::ConfigValidationError => "CONFIG_VALIDATION_ERROR",
                    Self::ConfigValidationWarning => "CONFIG_VALIDATION_WARNING",
                    Self::QuotaExceededBackendServices => {
                        "QUOTA_EXCEEDED_BACKEND_SERVICES"
                    }
                    Self::QuotaExceededHealthChecks => "QUOTA_EXCEEDED_HEALTH_CHECKS",
                    Self::QuotaExceededHttpRoutes => "QUOTA_EXCEEDED_HTTP_ROUTES",
                    Self::QuotaExceededTcpRoutes => "QUOTA_EXCEEDED_TCP_ROUTES",
                    Self::QuotaExceededTlsRoutes => "QUOTA_EXCEEDED_TLS_ROUTES",
                    Self::QuotaExceededTrafficPolicies => {
                        "QUOTA_EXCEEDED_TRAFFIC_POLICIES"
                    }
                    Self::QuotaExceededEndpointPolicies => {
                        "QUOTA_EXCEEDED_ENDPOINT_POLICIES"
                    }
                    Self::QuotaExceededGateways => "QUOTA_EXCEEDED_GATEWAYS",
                    Self::QuotaExceededMeshes => "QUOTA_EXCEEDED_MESHES",
                    Self::QuotaExceededServerTlsPolicies => {
                        "QUOTA_EXCEEDED_SERVER_TLS_POLICIES"
                    }
                    Self::QuotaExceededClientTlsPolicies => {
                        "QUOTA_EXCEEDED_CLIENT_TLS_POLICIES"
                    }
                    Self::QuotaExceededServiceLbPolicies => {
                        "QUOTA_EXCEEDED_SERVICE_LB_POLICIES"
                    }
                    Self::QuotaExceededHttpFilters => "QUOTA_EXCEEDED_HTTP_FILTERS",
                    Self::QuotaExceededTcpFilters => "QUOTA_EXCEEDED_TCP_FILTERS",
                    Self::QuotaExceededNetworkEndpointGroups => {
                        "QUOTA_EXCEEDED_NETWORK_ENDPOINT_GROUPS"
                    }
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "MESH_IAM_PERMISSION_DENIED" => Some(Self::MeshIamPermissionDenied),
                    "CNI_CONFIG_UNSUPPORTED" => Some(Self::CniConfigUnsupported),
                    "GKE_SANDBOX_UNSUPPORTED" => Some(Self::GkeSandboxUnsupported),
                    "NODEPOOL_WORKLOAD_IDENTITY_FEDERATION_REQUIRED" => {
                        Some(Self::NodepoolWorkloadIdentityFederationRequired)
                    }
                    "CNI_INSTALLATION_FAILED" => Some(Self::CniInstallationFailed),
                    "CNI_POD_UNSCHEDULABLE" => Some(Self::CniPodUnschedulable),
                    "UNSUPPORTED_MULTIPLE_CONTROL_PLANES" => {
                        Some(Self::UnsupportedMultipleControlPlanes)
                    }
                    "VPCSC_GA_SUPPORTED" => Some(Self::VpcscGaSupported),
                    "CONFIG_APPLY_INTERNAL_ERROR" => Some(Self::ConfigApplyInternalError),
                    "CONFIG_VALIDATION_ERROR" => Some(Self::ConfigValidationError),
                    "CONFIG_VALIDATION_WARNING" => Some(Self::ConfigValidationWarning),
                    "QUOTA_EXCEEDED_BACKEND_SERVICES" => {
                        Some(Self::QuotaExceededBackendServices)
                    }
                    "QUOTA_EXCEEDED_HEALTH_CHECKS" => {
                        Some(Self::QuotaExceededHealthChecks)
                    }
                    "QUOTA_EXCEEDED_HTTP_ROUTES" => Some(Self::QuotaExceededHttpRoutes),
                    "QUOTA_EXCEEDED_TCP_ROUTES" => Some(Self::QuotaExceededTcpRoutes),
                    "QUOTA_EXCEEDED_TLS_ROUTES" => Some(Self::QuotaExceededTlsRoutes),
                    "QUOTA_EXCEEDED_TRAFFIC_POLICIES" => {
                        Some(Self::QuotaExceededTrafficPolicies)
                    }
                    "QUOTA_EXCEEDED_ENDPOINT_POLICIES" => {
                        Some(Self::QuotaExceededEndpointPolicies)
                    }
                    "QUOTA_EXCEEDED_GATEWAYS" => Some(Self::QuotaExceededGateways),
                    "QUOTA_EXCEEDED_MESHES" => Some(Self::QuotaExceededMeshes),
                    "QUOTA_EXCEEDED_SERVER_TLS_POLICIES" => {
                        Some(Self::QuotaExceededServerTlsPolicies)
                    }
                    "QUOTA_EXCEEDED_CLIENT_TLS_POLICIES" => {
                        Some(Self::QuotaExceededClientTlsPolicies)
                    }
                    "QUOTA_EXCEEDED_SERVICE_LB_POLICIES" => {
                        Some(Self::QuotaExceededServiceLbPolicies)
                    }
                    "QUOTA_EXCEEDED_HTTP_FILTERS" => Some(Self::QuotaExceededHttpFilters),
                    "QUOTA_EXCEEDED_TCP_FILTERS" => Some(Self::QuotaExceededTcpFilters),
                    "QUOTA_EXCEEDED_NETWORK_ENDPOINT_GROUPS" => {
                        Some(Self::QuotaExceededNetworkEndpointGroups)
                    }
                    _ => None,
                }
            }
        }
        /// Severity level of the reported condition
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
        pub enum Severity {
            /// Unspecified severity
            Unspecified = 0,
            /// Indicates an issue that prevents the mesh from operating correctly
            Error = 1,
            /// Indicates a setting is likely wrong, but the mesh is still able to
            /// operate
            Warning = 2,
            /// An informational message, not requiring any action
            Info = 3,
        }
        impl Severity {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "SEVERITY_UNSPECIFIED",
                    Self::Error => "ERROR",
                    Self::Warning => "WARNING",
                    Self::Info => "INFO",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                    "ERROR" => Some(Self::Error),
                    "WARNING" => Some(Self::Warning),
                    "INFO" => Some(Self::Info),
                    _ => None,
                }
            }
        }
    }
    /// Lifecycle state of Service Mesh components.
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
    pub enum LifecycleState {
        /// Unspecified
        Unspecified = 0,
        /// DISABLED means that the component is not enabled.
        Disabled = 1,
        /// FAILED_PRECONDITION means that provisioning cannot proceed because of
        /// some characteristic of the member cluster.
        FailedPrecondition = 2,
        /// PROVISIONING means that provisioning is in progress.
        Provisioning = 3,
        /// ACTIVE means that the component is ready for use.
        Active = 4,
        /// STALLED means that provisioning could not be done.
        Stalled = 5,
        /// NEEDS_ATTENTION means that the component is ready, but some user
        /// intervention is required. (For example that the user should migrate
        /// workloads to a new control plane revision.)
        NeedsAttention = 6,
        /// DEGRADED means that the component is ready, but operating in a
        /// degraded state.
        Degraded = 7,
    }
    impl LifecycleState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "LIFECYCLE_STATE_UNSPECIFIED",
                Self::Disabled => "DISABLED",
                Self::FailedPrecondition => "FAILED_PRECONDITION",
                Self::Provisioning => "PROVISIONING",
                Self::Active => "ACTIVE",
                Self::Stalled => "STALLED",
                Self::NeedsAttention => "NEEDS_ATTENTION",
                Self::Degraded => "DEGRADED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIFECYCLE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DISABLED" => Some(Self::Disabled),
                "FAILED_PRECONDITION" => Some(Self::FailedPrecondition),
                "PROVISIONING" => Some(Self::Provisioning),
                "ACTIVE" => Some(Self::Active),
                "STALLED" => Some(Self::Stalled),
                "NEEDS_ATTENTION" => Some(Self::NeedsAttention),
                "DEGRADED" => Some(Self::Degraded),
                _ => None,
            }
        }
    }
}
/// Structured and human-readable details for a status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusDetails {
    /// A machine-readable code that further describes a broad status.
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
    /// Human-readable explanation of code.
    #[prost(string, tag = "2")]
    pub details: ::prost::alloc::string::String,
}

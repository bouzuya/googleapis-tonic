// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipState {
    #[prost(string, tag = "1")]
    pub cluster_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub membership_spec: ::core::option::Option<MembershipSpec>,
    #[prost(message, optional, tag = "3")]
    pub operator_state: ::core::option::Option<OperatorState>,
    #[prost(message, optional, tag = "4")]
    pub config_sync_state: ::core::option::Option<ConfigSyncState>,
    #[prost(message, optional, tag = "5")]
    pub policy_controller_state: ::core::option::Option<PolicyControllerState>,
    #[prost(message, optional, tag = "6")]
    pub binauthz_state: ::core::option::Option<BinauthzState>,
    #[prost(message, optional, tag = "7")]
    pub hierarchy_controller_state: ::core::option::Option<HierarchyControllerState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipSpec {
    #[prost(message, optional, tag = "1")]
    pub config_sync: ::core::option::Option<ConfigSync>,
    #[prost(message, optional, tag = "2")]
    pub policy_controller: ::core::option::Option<PolicyController>,
    #[prost(message, optional, tag = "3")]
    pub binauthz: ::core::option::Option<BinauthzConfig>,
    #[prost(message, optional, tag = "4")]
    pub hierarchy_controller: ::core::option::Option<HierarchyControllerConfig>,
    #[prost(string, tag = "10")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSync {
    #[prost(message, optional, tag = "7")]
    pub git: ::core::option::Option<GitConfig>,
    #[prost(string, tag = "8")]
    pub source_format: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitConfig {
    #[prost(string, tag = "1")]
    pub sync_repo: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sync_branch: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub policy_dir: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub sync_wait_secs: i64,
    #[prost(string, tag = "5")]
    pub sync_rev: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub secret_type: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub https_proxy: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub gcp_service_account_email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyController {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(bool, optional, tag = "2")]
    pub template_library_installed: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub audit_interval_seconds: ::core::option::Option<i64>,
    #[prost(string, repeated, tag = "4")]
    pub exemptable_namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "5")]
    pub referential_rules_enabled: bool,
    #[prost(bool, tag = "6")]
    pub log_denies_enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BinauthzConfig {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HierarchyControllerConfig {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(bool, tag = "2")]
    pub enable_pod_tree_labels: bool,
    #[prost(bool, tag = "3")]
    pub enable_hierarchical_resource_quota: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HierarchyControllerDeploymentState {
    #[prost(enumeration = "DeploymentState", tag = "1")]
    pub hnc: i32,
    #[prost(enumeration = "DeploymentState", tag = "2")]
    pub extension: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HierarchyControllerVersion {
    #[prost(string, tag = "1")]
    pub hnc: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub extension: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HierarchyControllerState {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<HierarchyControllerVersion>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<HierarchyControllerDeploymentState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorState {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(enumeration = "DeploymentState", tag = "2")]
    pub deployment_state: i32,
    #[prost(message, repeated, tag = "3")]
    pub errors: ::prost::alloc::vec::Vec<InstallError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallError {
    #[prost(string, tag = "1")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSyncState {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<ConfigSyncVersion>,
    #[prost(message, optional, tag = "2")]
    pub deployment_state: ::core::option::Option<ConfigSyncDeploymentState>,
    #[prost(message, optional, tag = "3")]
    pub sync_state: ::core::option::Option<SyncState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSyncVersion {
    #[prost(string, tag = "1")]
    pub importer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub syncer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub git_sync: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub monitor: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub reconciler_manager: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub root_reconciler: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ConfigSyncDeploymentState {
    #[prost(enumeration = "DeploymentState", tag = "1")]
    pub importer: i32,
    #[prost(enumeration = "DeploymentState", tag = "2")]
    pub syncer: i32,
    #[prost(enumeration = "DeploymentState", tag = "3")]
    pub git_sync: i32,
    #[prost(enumeration = "DeploymentState", tag = "4")]
    pub monitor: i32,
    #[prost(enumeration = "DeploymentState", tag = "5")]
    pub reconciler_manager: i32,
    #[prost(enumeration = "DeploymentState", tag = "6")]
    pub root_reconciler: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncState {
    #[prost(string, tag = "1")]
    pub source_token: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub import_token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sync_token: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "4")]
    pub last_sync: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub last_sync_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "sync_state::SyncCode", tag = "5")]
    pub code: i32,
    #[prost(message, repeated, tag = "6")]
    pub errors: ::prost::alloc::vec::Vec<SyncError>,
}
/// Nested message and enum types in `SyncState`.
pub mod sync_state {
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
    pub enum SyncCode {
        Unspecified = 0,
        Synced = 1,
        Pending = 2,
        Error = 3,
        NotConfigured = 4,
        NotInstalled = 5,
        Unauthorized = 6,
        Unreachable = 7,
    }
    impl SyncCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SyncCode::Unspecified => "SYNC_CODE_UNSPECIFIED",
                SyncCode::Synced => "SYNCED",
                SyncCode::Pending => "PENDING",
                SyncCode::Error => "ERROR",
                SyncCode::NotConfigured => "NOT_CONFIGURED",
                SyncCode::NotInstalled => "NOT_INSTALLED",
                SyncCode::Unauthorized => "UNAUTHORIZED",
                SyncCode::Unreachable => "UNREACHABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYNC_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "SYNCED" => Some(Self::Synced),
                "PENDING" => Some(Self::Pending),
                "ERROR" => Some(Self::Error),
                "NOT_CONFIGURED" => Some(Self::NotConfigured),
                "NOT_INSTALLED" => Some(Self::NotInstalled),
                "UNAUTHORIZED" => Some(Self::Unauthorized),
                "UNREACHABLE" => Some(Self::Unreachable),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncError {
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub error_resources: ::prost::alloc::vec::Vec<ErrorResource>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResource {
    #[prost(string, tag = "1")]
    pub source_path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub resource_namespace: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub resource_gvk: ::core::option::Option<GroupVersionKind>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupVersionKind {
    #[prost(string, tag = "1")]
    pub group: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyControllerState {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<PolicyControllerVersion>,
    #[prost(message, optional, tag = "2")]
    pub deployment_state: ::core::option::Option<GatekeeperDeploymentState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyControllerVersion {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinauthzState {
    #[prost(enumeration = "DeploymentState", tag = "1")]
    pub webhook: i32,
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<BinauthzVersion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinauthzVersion {
    #[prost(string, tag = "1")]
    pub webhook_version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GatekeeperDeploymentState {
    #[prost(enumeration = "DeploymentState", tag = "1")]
    pub gatekeeper_controller_manager_state: i32,
    #[prost(enumeration = "DeploymentState", tag = "2")]
    pub gatekeeper_audit: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeploymentState {
    Unspecified = 0,
    NotInstalled = 1,
    Installed = 2,
    Error = 3,
}
impl DeploymentState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeploymentState::Unspecified => "DEPLOYMENT_STATE_UNSPECIFIED",
            DeploymentState::NotInstalled => "NOT_INSTALLED",
            DeploymentState::Installed => "INSTALLED",
            DeploymentState::Error => "ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPLOYMENT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "NOT_INSTALLED" => Some(Self::NotInstalled),
            "INSTALLED" => Some(Self::Installed),
            "ERROR" => Some(Self::Error),
            _ => None,
        }
    }
}
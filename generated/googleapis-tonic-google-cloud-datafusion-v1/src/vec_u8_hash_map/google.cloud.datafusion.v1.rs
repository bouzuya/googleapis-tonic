// This file is @generated by prost-build.
/// Network configuration for a Data Fusion instance. These configurations
/// are used for peering with the customer network. Configurations are optional
/// when a public Data Fusion instance is to be created. However, providing
/// these configurations allows several benefits, such as reduced network latency
/// while accessing the customer resources from managed Data Fusion instance
/// nodes, as well as access to the customer on-prem resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Name of the network in the customer project with which the Tenant Project
    /// will be peered for executing pipelines. In case of shared VPC where the
    /// network resides in another host project the network should specified in
    /// the form of projects/{host-project-id}/global/networks/{network}
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// The IP range in CIDR notation to use for the managed Data Fusion instance
    /// nodes. This range must not overlap with any other ranges used in the
    /// customer network.
    #[prost(string, tag = "2")]
    pub ip_allocation: ::prost::alloc::string::String,
}
/// The Data Fusion version. This proto message stores information about certain
/// Data Fusion version, which is used for Data Fusion version upgrade.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// The version number of the Data Fusion instance, such as '6.0.1.0'.
    #[prost(string, tag = "1")]
    pub version_number: ::prost::alloc::string::String,
    /// Whether this is currently the default version for Cloud Data Fusion
    #[prost(bool, tag = "2")]
    pub default_version: bool,
    /// Represents a list of available feature names for a given version.
    #[prost(string, repeated, tag = "3")]
    pub available_features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Type represents the release availability of the version
    #[prost(enumeration = "version::Type", tag = "4")]
    pub r#type: i32,
}
/// Nested message and enum types in `Version`.
pub mod version {
    /// Each type represents the release availability of a CDF version
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
    pub enum Type {
        /// Version does not have availability yet
        Unspecified = 0,
        /// Version is under development and not considered stable
        Preview = 1,
        /// Version is available for public use
        GeneralAvailability = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "TYPE_UNSPECIFIED",
                Self::Preview => "TYPE_PREVIEW",
                Self::GeneralAvailability => "TYPE_GENERAL_AVAILABILITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TYPE_PREVIEW" => Some(Self::Preview),
                "TYPE_GENERAL_AVAILABILITY" => Some(Self::GeneralAvailability),
                _ => None,
            }
        }
    }
}
/// Identifies Data Fusion accelerators for an instance.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Accelerator {
    /// The type of an accelator for a CDF instance.
    #[prost(enumeration = "accelerator::AcceleratorType", tag = "1")]
    pub accelerator_type: i32,
    /// The state of the accelerator
    #[prost(enumeration = "accelerator::State", tag = "2")]
    pub state: i32,
}
/// Nested message and enum types in `Accelerator`.
pub mod accelerator {
    /// Each type represents an Accelerator (Add-On) supported by Cloud Data Fusion
    /// service.
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
    pub enum AcceleratorType {
        /// Default value, if unspecified.
        Unspecified = 0,
        /// Change Data Capture accelerator for CDF.
        Cdc = 1,
        /// Cloud Healthcare accelerator for CDF. This accelerator is to enable Cloud
        /// Healthcare specific CDF plugins developed by Healthcare team.
        Healthcare = 2,
        /// Contact Center AI Insights
        /// This accelerator is used to enable import and export pipelines
        /// custom built to streamline CCAI Insights processing.
        CcaiInsights = 3,
    }
    impl AcceleratorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "ACCELERATOR_TYPE_UNSPECIFIED",
                Self::Cdc => "CDC",
                Self::Healthcare => "HEALTHCARE",
                Self::CcaiInsights => "CCAI_INSIGHTS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCELERATOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CDC" => Some(Self::Cdc),
                "HEALTHCARE" => Some(Self::Healthcare),
                "CCAI_INSIGHTS" => Some(Self::CcaiInsights),
                _ => None,
            }
        }
    }
    /// Different values possible for the state of an accelerator
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
        /// Default value, do not use
        Unspecified = 0,
        /// Indicates that the accelerator is enabled and available to use
        Enabled = 1,
        /// Indicates that the accelerator is disabled and not available to use
        Disabled = 2,
        /// Indicates that accelerator state is currently unknown.
        /// Requests for enable, disable could be retried while in this state
        Unknown = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Enabled => "ENABLED",
                Self::Disabled => "DISABLED",
                Self::Unknown => "UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "UNKNOWN" => Some(Self::Unknown),
                _ => None,
            }
        }
    }
}
/// The crypto key configuration. This field is used by the Customer-managed
/// encryption keys (CMEK) feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyConfig {
    /// The name of the key which is used to encrypt/decrypt customer data. For key
    /// in Cloud KMS, the key should be in the format of
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub key_reference: ::prost::alloc::string::String,
}
/// Represents a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The name of this instance is in the form of
    /// projects/{project}/locations/{location}/instances/{instance}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of this instance.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Instance type.
    #[prost(enumeration = "instance::Type", tag = "3")]
    pub r#type: i32,
    /// Option to enable Stackdriver Logging.
    #[prost(bool, tag = "4")]
    pub enable_stackdriver_logging: bool,
    /// Option to enable Stackdriver Monitoring.
    #[prost(bool, tag = "5")]
    pub enable_stackdriver_monitoring: bool,
    /// Specifies whether the Data Fusion instance should be private. If set to
    /// true, all Data Fusion nodes will have private IP addresses and will not be
    /// able to access the public internet.
    #[prost(bool, tag = "6")]
    pub private_instance: bool,
    /// Network configuration options. These are required when a private Data
    /// Fusion instance is to be created.
    #[prost(message, optional, tag = "7")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// The resource labels for instance to use to annotate any related underlying
    /// resources such as Compute Engine VMs. The character '=' is not allowed to
    /// be used within the labels.
    #[prost(map = "string, string", tag = "8")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Map of additional options used to configure the behavior of
    /// Data Fusion instance.
    #[prost(map = "string, string", tag = "9")]
    pub options: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the instance was last updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this Data Fusion instance.
    #[prost(enumeration = "instance::State", tag = "12")]
    pub state: i32,
    /// Output only. Additional information about the current state of this Data
    /// Fusion instance if available.
    #[prost(string, tag = "13")]
    pub state_message: ::prost::alloc::string::String,
    /// Output only. Endpoint on which the Data Fusion UI is accessible.
    #[prost(string, tag = "14")]
    pub service_endpoint: ::prost::alloc::string::String,
    /// Name of the zone in which the Data Fusion instance will be created. Only
    /// DEVELOPER instances use this field.
    #[prost(string, tag = "15")]
    pub zone: ::prost::alloc::string::String,
    /// Current version of the Data Fusion. Only specifiable in Update.
    #[prost(string, tag = "16")]
    pub version: ::prost::alloc::string::String,
    /// Output only. Deprecated. Use tenant_project_id instead to extract the tenant project ID.
    #[deprecated]
    #[prost(string, tag = "17")]
    pub service_account: ::prost::alloc::string::String,
    /// Display name for an instance.
    #[prost(string, tag = "18")]
    pub display_name: ::prost::alloc::string::String,
    /// Available versions that the instance can be upgraded to using
    /// UpdateInstanceRequest.
    #[prost(message, repeated, tag = "19")]
    pub available_version: ::prost::alloc::vec::Vec<Version>,
    /// Output only. Endpoint on which the REST APIs is accessible.
    #[prost(string, tag = "20")]
    pub api_endpoint: ::prost::alloc::string::String,
    /// Output only. Cloud Storage bucket generated by Data Fusion in the customer project.
    #[prost(string, tag = "21")]
    pub gcs_bucket: ::prost::alloc::string::String,
    /// List of accelerators enabled for this CDF instance.
    #[prost(message, repeated, tag = "22")]
    pub accelerators: ::prost::alloc::vec::Vec<Accelerator>,
    /// Output only. P4 service account for the customer project.
    #[prost(string, tag = "23")]
    pub p4_service_account: ::prost::alloc::string::String,
    /// Output only. The name of the tenant project.
    #[prost(string, tag = "24")]
    pub tenant_project_id: ::prost::alloc::string::String,
    /// User-managed service account to set on Dataproc when Cloud Data Fusion
    /// creates Dataproc to run data processing pipelines.
    ///
    /// This allows users to have fine-grained access control on Dataproc's
    /// accesses to cloud resources.
    #[prost(string, tag = "25")]
    pub dataproc_service_account: ::prost::alloc::string::String,
    /// Option to enable granular role-based access control.
    #[prost(bool, tag = "27")]
    pub enable_rbac: bool,
    /// The crypto key configuration. This field is used by the Customer-Managed
    /// Encryption Keys (CMEK) feature.
    #[prost(message, optional, tag = "28")]
    pub crypto_key_config: ::core::option::Option<CryptoKeyConfig>,
    /// Output only. If the instance state is DISABLED, the reason for disabling the instance.
    #[prost(
        enumeration = "instance::DisabledReason",
        repeated,
        packed = "false",
        tag = "29"
    )]
    pub disabled_reason: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Represents the type of Data Fusion instance. Each type is configured with
    /// the default settings for processing and memory.
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
    pub enum Type {
        /// No type specified. The instance creation will fail.
        Unspecified = 0,
        /// Basic Data Fusion instance. In Basic type, the user will be able to
        /// create data pipelines using point and click UI. However, there are
        /// certain limitations, such as fewer number of concurrent pipelines, no
        /// support for streaming pipelines, etc.
        Basic = 1,
        /// Enterprise Data Fusion instance. In Enterprise type, the user will have
        /// all features available, such as support for streaming pipelines, higher
        /// number of concurrent pipelines, etc.
        Enterprise = 2,
        /// Developer Data Fusion instance. In Developer type, the user will have all
        /// features available but with restrictive capabilities. This is to help
        /// enterprises design and develop their data ingestion and integration
        /// pipelines at low cost.
        Developer = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "TYPE_UNSPECIFIED",
                Self::Basic => "BASIC",
                Self::Enterprise => "ENTERPRISE",
                Self::Developer => "DEVELOPER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BASIC" => Some(Self::Basic),
                "ENTERPRISE" => Some(Self::Enterprise),
                "DEVELOPER" => Some(Self::Developer),
                _ => None,
            }
        }
    }
    /// Represents the state of a Data Fusion instance
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
        /// Instance does not have a state yet
        Unspecified = 0,
        /// Instance is being created
        Creating = 1,
        /// Instance is active and ready for requests. This corresponds to 'RUNNING'
        /// in datafusion.v1beta1.
        Active = 2,
        /// Instance creation failed
        Failed = 3,
        /// Instance is being deleted
        Deleting = 4,
        /// Instance is being upgraded
        Upgrading = 5,
        /// Instance is being restarted
        Restarting = 6,
        /// Instance is being updated on customer request
        Updating = 7,
        /// Instance is being auto-updated
        AutoUpdating = 8,
        /// Instance is being auto-upgraded
        AutoUpgrading = 9,
        /// Instance is disabled
        Disabled = 10,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Creating => "CREATING",
                Self::Active => "ACTIVE",
                Self::Failed => "FAILED",
                Self::Deleting => "DELETING",
                Self::Upgrading => "UPGRADING",
                Self::Restarting => "RESTARTING",
                Self::Updating => "UPDATING",
                Self::AutoUpdating => "AUTO_UPDATING",
                Self::AutoUpgrading => "AUTO_UPGRADING",
                Self::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "FAILED" => Some(Self::Failed),
                "DELETING" => Some(Self::Deleting),
                "UPGRADING" => Some(Self::Upgrading),
                "RESTARTING" => Some(Self::Restarting),
                "UPDATING" => Some(Self::Updating),
                "AUTO_UPDATING" => Some(Self::AutoUpdating),
                "AUTO_UPGRADING" => Some(Self::AutoUpgrading),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
    /// The reason for disabling the instance if the state is DISABLED.
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
    pub enum DisabledReason {
        /// This is an unknown reason for disabling.
        Unspecified = 0,
        /// The KMS key used by the instance is either revoked or denied access to
        KmsKeyIssue = 1,
    }
    impl DisabledReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "DISABLED_REASON_UNSPECIFIED",
                Self::KmsKeyIssue => "KMS_KEY_ISSUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISABLED_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "KMS_KEY_ISSUE" => Some(Self::KmsKeyIssue),
                _ => None,
            }
        }
    }
}
/// Request message for listing Data Fusion instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The project and location for which to retrieve instance information
    /// in the format projects/{project}/locations/{location}. If the location is
    /// specified as '-' (wildcard), then all regions available to the project
    /// are queried, and the results are aggregated.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc",  or "" (unsorted).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for the list instance request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// Represents a list of Data Fusion instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Token to retrieve the next page of results or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for the list available versions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAvailableVersionsRequest {
    /// Required. The project and location for which to retrieve instance information
    /// in the format projects/{project}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether or not to return the latest patch of every available minor version.
    /// If true, only the latest patch will be returned. Ex. if allowed versions is
    /// \[6.1.1, 6.1.2, 6.2.0\] then response will be \[6.1.2, 6.2.0\]
    #[prost(bool, tag = "4")]
    pub latest_patch_only: bool,
}
/// Response message for the list available versions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAvailableVersionsResponse {
    /// Represents a list of versions that are supported.
    #[prost(message, repeated, tag = "1")]
    pub available_versions: ::prost::alloc::vec::Vec<Version>,
    /// Token to retrieve the next page of results or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for getting details about a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The instance resource name in the format
    /// projects/{project}/locations/{location}/instances/{instance}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for creating a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The instance's project and location in the format
    /// projects/{project}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The name of the instance to create.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// An instance resource.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// Request message for deleting a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The instance resource name in the format
    /// projects/{project}/locations/{location}/instances/{instance}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for updating a Data Fusion instance.
/// Data Fusion allows updating the labels, options, and stack driver settings.
/// This is also used for CDF version upgrade.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. The instance resource that replaces the resource on the server. Currently,
    /// Data Fusion only allows replacing labels, options, and stack driver
    /// settings. All other fields will be ignored.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// Field mask is used to specify the fields that the update will overwrite
    /// in an instance resource. The fields specified in the update_mask are
    /// relative to the resource, not the full request.
    /// A field will be overwritten if it is in the mask.
    /// If the user does not provide a mask, all the supported fields (labels,
    /// options, and version currently) will be overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for restarting a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartInstanceRequest {
    /// Required. Name of the Data Fusion instance which need to be restarted in the form of
    /// projects/{project}/locations/{location}/instances/{instance}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// Map to hold any additional status info for the operation
    /// If there is an accelerator being enabled/disabled/deleted, this will be
    /// populated with accelerator name as key and status as
    /// ENABLING, DISABLING or DELETING
    #[prost(map = "string, string", tag = "8")]
    pub additional_status: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Generated client implementations.
pub mod data_fusion_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for creating and managing Data Fusion instances.
    /// Data Fusion enables ETL developers to build code-free, data integration
    /// pipelines via a point-and-click UI.
    #[derive(Debug, Clone)]
    pub struct DataFusionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataFusionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> DataFusionClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            DataFusionClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists possible versions for Data Fusion instances in the specified project
        /// and location.
        pub async fn list_available_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAvailableVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAvailableVersionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1.DataFusion/ListAvailableVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datafusion.v1.DataFusion",
                        "ListAvailableVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Data Fusion instances in the specified project and location.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstancesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1.DataFusion/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datafusion.v1.DataFusion",
                        "ListInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Data Fusion instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1.DataFusion/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datafusion.v1.DataFusion",
                        "GetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Data Fusion instance in the specified project and location.
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1.DataFusion/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datafusion.v1.DataFusion",
                        "CreateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Date Fusion instance.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1.DataFusion/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datafusion.v1.DataFusion",
                        "DeleteInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a single Data Fusion instance.
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1.DataFusion/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datafusion.v1.DataFusion",
                        "UpdateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restart a single Data Fusion instance.
        /// At the end of an operation instance is fully restarted.
        pub async fn restart_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RestartInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1.DataFusion/RestartInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datafusion.v1.DataFusion",
                        "RestartInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}

// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag = "1")]
    pub event_time: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub srcid: i64,
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub event_id: i32,
    #[prost(string, tag = "5")]
    pub component: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub appliance_name: i64,
    #[prost(string, tag = "7")]
    pub app_name: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub app_type: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub job_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRecoveryJobReportLog {
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub job_category: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub job_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub log_backup: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub job_status: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub error_code: i32,
    #[prost(string, tag = "9")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub job_initiation_failure_reason: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub job_start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub job_end_time: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub job_queued_time: ::prost::alloc::string::String,
    #[prost(double, tag = "14")]
    pub job_duration_in_hours: f64,
    #[prost(string, tag = "15")]
    pub hostname: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub appliance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub backup_rule_policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub backup_plan_policy_template: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub backup_type: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub recovery_point: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub backup_consistency: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub target_host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub target_appliance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "24")]
    pub target_pool_name: ::prost::alloc::string::String,
    #[prost(double, tag = "25")]
    pub resource_data_size_in_gib: f64,
    #[prost(double, tag = "26")]
    pub data_copied_in_gib: f64,
    #[prost(double, tag = "27")]
    pub onvault_pool_storage_consumed_in_gib: f64,
    #[prost(double, tag = "28")]
    pub pre_compress_in_gib: f64,
    #[prost(double, tag = "29")]
    pub compression_ratio: f64,
    #[prost(double, tag = "30")]
    pub data_change_rate: f64,
    #[prost(double, tag = "31")]
    pub snapshot_disk_size_in_gib: f64,
    #[prost(double, tag = "32")]
    pub data_written_in_gib: f64,
    #[prost(double, tag = "33")]
    pub data_sent_in_gib: f64,
    #[prost(string, tag = "34")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(string, tag = "35")]
    pub host_id: ::prost::alloc::string::String,
    #[prost(string, tag = "36")]
    pub backup_rule_policy_id: ::prost::alloc::string::String,
    #[prost(string, tag = "37")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "38")]
    pub target_pool_id: ::prost::alloc::string::String,
    #[prost(string, tag = "39")]
    pub target_host_id: ::prost::alloc::string::String,
    #[prost(string, tag = "40")]
    pub target_appliance_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnprotectedResourceReportLog {
    #[prost(string, tag = "1")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub discovered_on: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub discovered_by: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub appliance_id: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub host_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyScheduleComplianceReportLog {
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub backup_rule_policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub backup_plan_policy_template: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub appliance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub date: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub backup_window_start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub job_type: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub comment: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub host_id: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub backup_plan_policy_template_id: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub backup_rule_policy_id: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub appliance_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupStorageUtilizationReportLog {
    #[prost(string, tag = "1")]
    pub appliance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub storage_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pool_name: ::prost::alloc::string::String,
    #[prost(double, tag = "4")]
    pub total_capacity_in_gib: f64,
    #[prost(double, tag = "5")]
    pub used_capacity_in_gib: f64,
    #[prost(double, tag = "6")]
    pub utilization_percentage: f64,
    #[prost(string, tag = "7")]
    pub appliance_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectedResource {
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub backup_inclusion_or_exclusion: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub host_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub backup_plan_policy_template_id: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub backup_plan_policy_template: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub sla_id: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub backup_plan_restrictions: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub protected_on: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub policy_overrides: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub source_appliance: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub source_appliance_id: ::prost::alloc::string::String,
    #[prost(double, tag = "15")]
    pub protected_data_in_gib: f64,
    #[prost(double, tag = "16")]
    pub onvault_in_gib: f64,
    #[prost(string, tag = "17")]
    pub appliance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub appliance_id: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub remote_appliance: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub remote_appliance_id: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub recovery_point: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountedImage {
    #[prost(string, tag = "1")]
    pub source_resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub appliance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub appliance_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub mounted_image_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub source_image_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub job_type: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub recovery_point_date: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub last_mount_date: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub source_host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub source_host_id: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub mounted_host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub mounted_host_id: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub mounted_resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub resource_virtual_size: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub storage_consumed: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub mounted_resource_label: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub restorable_object: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub mount_duration: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub read_mode: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub resource_size: ::prost::alloc::string::String,
    #[prost(string, tag = "24")]
    pub image_expiration_date: ::prost::alloc::string::String,
}
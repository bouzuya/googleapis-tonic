// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "policy::GlobalPolicyEvaluationMode", tag = "7")]
    pub global_policy_evaluation_mode: i32,
    #[prost(message, repeated, tag = "2")]
    pub admission_whitelist_patterns: ::prost::alloc::vec::Vec<
        AdmissionWhitelistPattern,
    >,
    #[prost(btree_map = "string, message", tag = "3")]
    pub cluster_admission_rules: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        AdmissionRule,
    >,
    #[prost(btree_map = "string, message", tag = "10")]
    pub kubernetes_namespace_admission_rules: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        AdmissionRule,
    >,
    #[prost(btree_map = "string, message", tag = "8")]
    pub kubernetes_service_account_admission_rules: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        AdmissionRule,
    >,
    #[prost(btree_map = "string, message", tag = "9")]
    pub istio_service_identity_admission_rules: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        AdmissionRule,
    >,
    #[prost(message, optional, tag = "4")]
    pub default_admission_rule: ::core::option::Option<AdmissionRule>,
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Policy`.
pub mod policy {
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
    pub enum GlobalPolicyEvaluationMode {
        Unspecified = 0,
        Enable = 1,
        Disable = 2,
    }
    impl GlobalPolicyEvaluationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GlobalPolicyEvaluationMode::Unspecified => {
                    "GLOBAL_POLICY_EVALUATION_MODE_UNSPECIFIED"
                }
                GlobalPolicyEvaluationMode::Enable => "ENABLE",
                GlobalPolicyEvaluationMode::Disable => "DISABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GLOBAL_POLICY_EVALUATION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLE" => Some(Self::Enable),
                "DISABLE" => Some(Self::Disable),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionWhitelistPattern {
    #[prost(string, tag = "1")]
    pub name_pattern: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionRule {
    #[prost(enumeration = "admission_rule::EvaluationMode", tag = "1")]
    pub evaluation_mode: i32,
    #[prost(string, repeated, tag = "2")]
    pub require_attestations_by: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(enumeration = "admission_rule::EnforcementMode", tag = "3")]
    pub enforcement_mode: i32,
}
/// Nested message and enum types in `AdmissionRule`.
pub mod admission_rule {
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
    pub enum EvaluationMode {
        Unspecified = 0,
        AlwaysAllow = 1,
        RequireAttestation = 2,
        AlwaysDeny = 3,
    }
    impl EvaluationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EvaluationMode::Unspecified => "EVALUATION_MODE_UNSPECIFIED",
                EvaluationMode::AlwaysAllow => "ALWAYS_ALLOW",
                EvaluationMode::RequireAttestation => "REQUIRE_ATTESTATION",
                EvaluationMode::AlwaysDeny => "ALWAYS_DENY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVALUATION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ALWAYS_ALLOW" => Some(Self::AlwaysAllow),
                "REQUIRE_ATTESTATION" => Some(Self::RequireAttestation),
                "ALWAYS_DENY" => Some(Self::AlwaysDeny),
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
    pub enum EnforcementMode {
        Unspecified = 0,
        EnforcedBlockAndAuditLog = 1,
        DryrunAuditLogOnly = 2,
    }
    impl EnforcementMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnforcementMode::Unspecified => "ENFORCEMENT_MODE_UNSPECIFIED",
                EnforcementMode::EnforcedBlockAndAuditLog => {
                    "ENFORCED_BLOCK_AND_AUDIT_LOG"
                }
                EnforcementMode::DryrunAuditLogOnly => "DRYRUN_AUDIT_LOG_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENFORCEMENT_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENFORCED_BLOCK_AND_AUDIT_LOG" => Some(Self::EnforcedBlockAndAuditLog),
                "DRYRUN_AUDIT_LOG_ONLY" => Some(Self::DryrunAuditLogOnly),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attestor {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "attestor::AttestorType", tags = "3")]
    pub attestor_type: ::core::option::Option<attestor::AttestorType>,
}
/// Nested message and enum types in `Attestor`.
pub mod attestor {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AttestorType {
        #[prost(message, tag = "3")]
        UserOwnedGrafeasNote(super::UserOwnedGrafeasNote),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOwnedGrafeasNote {
    #[prost(string, tag = "1")]
    pub note_reference: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub public_keys: ::prost::alloc::vec::Vec<AttestorPublicKey>,
    #[prost(string, tag = "3")]
    pub delegation_service_account_email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkixPublicKey {
    #[prost(string, tag = "1")]
    pub public_key_pem: ::prost::alloc::string::String,
    #[prost(enumeration = "pkix_public_key::SignatureAlgorithm", tag = "2")]
    pub signature_algorithm: i32,
}
/// Nested message and enum types in `PkixPublicKey`.
pub mod pkix_public_key {
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
    pub enum SignatureAlgorithm {
        Unspecified = 0,
        RsaPss2048Sha256 = 1,
        RsaPss3072Sha256 = 2,
        RsaPss4096Sha256 = 3,
        RsaPss4096Sha512 = 4,
        RsaSignPkcs12048Sha256 = 5,
        RsaSignPkcs13072Sha256 = 6,
        RsaSignPkcs14096Sha256 = 7,
        RsaSignPkcs14096Sha512 = 8,
        EcdsaP256Sha256 = 9,
        EcdsaP384Sha384 = 10,
        EcdsaP521Sha512 = 11,
    }
    impl SignatureAlgorithm {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SignatureAlgorithm::Unspecified => "SIGNATURE_ALGORITHM_UNSPECIFIED",
                SignatureAlgorithm::RsaPss2048Sha256 => "RSA_PSS_2048_SHA256",
                SignatureAlgorithm::RsaPss3072Sha256 => "RSA_PSS_3072_SHA256",
                SignatureAlgorithm::RsaPss4096Sha256 => "RSA_PSS_4096_SHA256",
                SignatureAlgorithm::RsaPss4096Sha512 => "RSA_PSS_4096_SHA512",
                SignatureAlgorithm::RsaSignPkcs12048Sha256 => {
                    "RSA_SIGN_PKCS1_2048_SHA256"
                }
                SignatureAlgorithm::RsaSignPkcs13072Sha256 => {
                    "RSA_SIGN_PKCS1_3072_SHA256"
                }
                SignatureAlgorithm::RsaSignPkcs14096Sha256 => {
                    "RSA_SIGN_PKCS1_4096_SHA256"
                }
                SignatureAlgorithm::RsaSignPkcs14096Sha512 => {
                    "RSA_SIGN_PKCS1_4096_SHA512"
                }
                SignatureAlgorithm::EcdsaP256Sha256 => "ECDSA_P256_SHA256",
                SignatureAlgorithm::EcdsaP384Sha384 => "ECDSA_P384_SHA384",
                SignatureAlgorithm::EcdsaP521Sha512 => "ECDSA_P521_SHA512",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SIGNATURE_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
                "RSA_PSS_2048_SHA256" => Some(Self::RsaPss2048Sha256),
                "RSA_PSS_3072_SHA256" => Some(Self::RsaPss3072Sha256),
                "RSA_PSS_4096_SHA256" => Some(Self::RsaPss4096Sha256),
                "RSA_PSS_4096_SHA512" => Some(Self::RsaPss4096Sha512),
                "RSA_SIGN_PKCS1_2048_SHA256" => Some(Self::RsaSignPkcs12048Sha256),
                "RSA_SIGN_PKCS1_3072_SHA256" => Some(Self::RsaSignPkcs13072Sha256),
                "RSA_SIGN_PKCS1_4096_SHA256" => Some(Self::RsaSignPkcs14096Sha256),
                "RSA_SIGN_PKCS1_4096_SHA512" => Some(Self::RsaSignPkcs14096Sha512),
                "ECDSA_P256_SHA256" => Some(Self::EcdsaP256Sha256),
                "ECDSA_P384_SHA384" => Some(Self::EcdsaP384Sha384),
                "ECDSA_P521_SHA512" => Some(Self::EcdsaP521Sha512),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestorPublicKey {
    #[prost(string, tag = "1")]
    pub comment: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "attestor_public_key::PublicKey", tags = "3, 5")]
    pub public_key: ::core::option::Option<attestor_public_key::PublicKey>,
}
/// Nested message and enum types in `AttestorPublicKey`.
pub mod attestor_public_key {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PublicKey {
        #[prost(string, tag = "3")]
        AsciiArmoredPgpPublicKey(::prost::alloc::string::String),
        #[prost(message, tag = "5")]
        PkixPublicKey(super::PkixPublicKey),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyRequest {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<Policy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAttestorRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub attestor_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub attestor: ::core::option::Option<Attestor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAttestorRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAttestorRequest {
    #[prost(message, optional, tag = "1")]
    pub attestor: ::core::option::Option<Attestor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttestorsRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttestorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub attestors: ::prost::alloc::vec::Vec<Attestor>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAttestorRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemPolicyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateAttestationOccurrenceRequest {
    #[prost(string, tag = "1")]
    pub attestor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub attestation: ::core::option::Option<
        super::super::super::super::grafeas::v1::AttestationOccurrence,
    >,
    #[prost(string, tag = "3")]
    pub occurrence_note: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub occurrence_resource_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateAttestationOccurrenceResponse {
    #[prost(enumeration = "validate_attestation_occurrence_response::Result", tag = "1")]
    pub result: i32,
    #[prost(string, tag = "2")]
    pub denial_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ValidateAttestationOccurrenceResponse`.
pub mod validate_attestation_occurrence_response {
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
    pub enum Result {
        Unspecified = 0,
        Verified = 1,
        AttestationNotVerifiable = 2,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unspecified => "RESULT_UNSPECIFIED",
                Result::Verified => "VERIFIED",
                Result::AttestationNotVerifiable => "ATTESTATION_NOT_VERIFIABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNSPECIFIED" => Some(Self::Unspecified),
                "VERIFIED" => Some(Self::Verified),
                "ATTESTATION_NOT_VERIFIABLE" => Some(Self::AttestationNotVerifiable),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod binauthz_management_service_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Cloud Management Service for Binary Authorization admission policies
    /// and attestation authorities.
    ///
    /// This API implements a REST model with the following objects:
    ///
    /// * [Policy][google.cloud.binaryauthorization.v1.Policy]
    /// * [Attestor][google.cloud.binaryauthorization.v1.Attestor]
    #[derive(Debug, Clone)]
    pub struct BinauthzManagementServiceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BinauthzManagementServiceV1Client<T>
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
        ) -> BinauthzManagementServiceV1Client<InterceptedService<T, F>>
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
            BinauthzManagementServiceV1Client::new(
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
        /// A [policy][google.cloud.binaryauthorization.v1.Policy] specifies the [attestors][google.cloud.binaryauthorization.v1.Attestor] that must attest to
        /// a container image, before the project is allowed to deploy that
        /// image. There is at most one policy per project. All image admission
        /// requests are permitted if a project has no policy.
        ///
        /// Gets the [policy][google.cloud.binaryauthorization.v1.Policy] for this project. Returns a default
        /// [policy][google.cloud.binaryauthorization.v1.Policy] if the project does not have one.
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/GetPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1",
                        "GetPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates a project's [policy][google.cloud.binaryauthorization.v1.Policy], and returns a copy of the
        /// new [policy][google.cloud.binaryauthorization.v1.Policy]. A policy is always updated as a whole, to avoid race
        /// conditions with concurrent policy enforcement (or management!)
        /// requests. Returns NOT_FOUND if the project does not exist, INVALID_ARGUMENT
        /// if the request is malformed.
        pub async fn update_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/UpdatePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1",
                        "UpdatePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an [attestor][google.cloud.binaryauthorization.v1.Attestor], and returns a copy of the new
        /// [attestor][google.cloud.binaryauthorization.v1.Attestor]. Returns NOT_FOUND if the project does not exist,
        /// INVALID_ARGUMENT if the request is malformed, ALREADY_EXISTS if the
        /// [attestor][google.cloud.binaryauthorization.v1.Attestor] already exists.
        pub async fn create_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAttestorRequest>,
        ) -> std::result::Result<tonic::Response<super::Attestor>, tonic::Status> {
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
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/CreateAttestor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1",
                        "CreateAttestor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an [attestor][google.cloud.binaryauthorization.v1.Attestor].
        /// Returns NOT_FOUND if the [attestor][google.cloud.binaryauthorization.v1.Attestor] does not exist.
        pub async fn get_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAttestorRequest>,
        ) -> std::result::Result<tonic::Response<super::Attestor>, tonic::Status> {
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
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/GetAttestor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1",
                        "GetAttestor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an [attestor][google.cloud.binaryauthorization.v1.Attestor].
        /// Returns NOT_FOUND if the [attestor][google.cloud.binaryauthorization.v1.Attestor] does not exist.
        pub async fn update_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAttestorRequest>,
        ) -> std::result::Result<tonic::Response<super::Attestor>, tonic::Status> {
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
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/UpdateAttestor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1",
                        "UpdateAttestor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists [attestors][google.cloud.binaryauthorization.v1.Attestor].
        /// Returns INVALID_ARGUMENT if the project does not exist.
        pub async fn list_attestors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAttestorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAttestorsResponse>,
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
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/ListAttestors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1",
                        "ListAttestors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an [attestor][google.cloud.binaryauthorization.v1.Attestor]. Returns NOT_FOUND if the
        /// [attestor][google.cloud.binaryauthorization.v1.Attestor] does not exist.
        pub async fn delete_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAttestorRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/DeleteAttestor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1",
                        "DeleteAttestor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod system_policy_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for working with the system policy.
    #[derive(Debug, Clone)]
    pub struct SystemPolicyV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SystemPolicyV1Client<T>
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
        ) -> SystemPolicyV1Client<InterceptedService<T, F>>
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
            SystemPolicyV1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Gets the current system policy in the specified location.
        pub async fn get_system_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSystemPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.cloud.binaryauthorization.v1.SystemPolicyV1/GetSystemPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.SystemPolicyV1",
                        "GetSystemPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod validation_helper_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// BinAuthz Attestor verification
    #[derive(Debug, Clone)]
    pub struct ValidationHelperV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ValidationHelperV1Client<T>
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
        ) -> ValidationHelperV1Client<InterceptedService<T, F>>
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
            ValidationHelperV1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Returns whether the given Attestation for the given image URI
        /// was signed by the given Attestor
        pub async fn validate_attestation_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateAttestationOccurrenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateAttestationOccurrenceResponse>,
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
                "/google.cloud.binaryauthorization.v1.ValidationHelperV1/ValidateAttestationOccurrence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.binaryauthorization.v1.ValidationHelperV1",
                        "ValidateAttestationOccurrence",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
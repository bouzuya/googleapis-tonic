// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Folder {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(enumeration = "folder::LifecycleState", tag = "4")]
    pub lifecycle_state: i32,
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Folder`.
pub mod folder {
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
        Unspecified = 0,
        Active = 1,
        DeleteRequested = 2,
    }
    impl LifecycleState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LifecycleState::Unspecified => "LIFECYCLE_STATE_UNSPECIFIED",
                LifecycleState::Active => "ACTIVE",
                LifecycleState::DeleteRequested => "DELETE_REQUESTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIFECYCLE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "DELETE_REQUESTED" => Some(Self::DeleteRequested),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersResponse {
    #[prost(message, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFoldersRequest {
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFoldersResponse {
    #[prost(message, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFolderRequest {
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub folder: ::core::option::Option<Folder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFolderRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub destination_parent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFolderRequest {
    #[prost(message, optional, tag = "1")]
    pub folder: ::core::option::Option<Folder>,
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFolderRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub recursive_delete: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteFolderRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderOperation {
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(enumeration = "folder_operation::OperationType", tag = "2")]
    pub operation_type: i32,
    #[prost(string, tag = "3")]
    pub source_parent: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub destination_parent: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FolderOperation`.
pub mod folder_operation {
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
    pub enum OperationType {
        Unspecified = 0,
        Create = 1,
        Move = 2,
    }
    impl OperationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
                OperationType::Create => "CREATE",
                OperationType::Move => "MOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE" => Some(Self::Create),
                "MOVE" => Some(Self::Move),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod folders_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages Cloud Resource Folders.
    /// Cloud Resource Folders can be used to organize the resources under an
    /// organization and to control the IAM policies applied to groups of resources.
    #[derive(Debug, Clone)]
    pub struct FoldersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FoldersClient<T>
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
        ) -> FoldersClient<InterceptedService<T, F>>
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
            FoldersClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the Folders that are direct descendants of supplied parent resource.
        /// List provides a strongly consistent view of the Folders underneath
        /// the specified parent resource.
        /// List returns Folders sorted based upon the (ascending) lexical ordering
        /// of their display_name.
        /// The caller must have `resourcemanager.folders.list` permission on the
        /// identified parent.
        pub async fn list_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFoldersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFoldersResponse>,
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
                "/google.cloud.resourcemanager.v2.Folders/ListFolders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "ListFolders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Search for folders that match specific filter criteria.
        /// Search provides an eventually consistent view of the folders a user has
        /// access to which meet the specified filter criteria.
        ///
        /// This will only return folders on which the caller has the
        /// permission `resourcemanager.folders.get`.
        pub async fn search_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchFoldersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchFoldersResponse>,
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
                "/google.cloud.resourcemanager.v2.Folders/SearchFolders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "SearchFolders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a Folder identified by the supplied resource name.
        /// Valid Folder resource names have the format `folders/{folder_id}`
        /// (for example, `folders/1234`).
        /// The caller must have `resourcemanager.folders.get` permission on the
        /// identified folder.
        pub async fn get_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::Folder>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v2.Folders/GetFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "GetFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Folder in the resource hierarchy.
        /// Returns an Operation which can be used to track the progress of the
        /// folder creation workflow.
        /// Upon success the Operation.response field will be populated with the
        /// created Folder.
        ///
        /// In order to succeed, the addition of this new Folder must not violate
        /// the Folder naming, height or fanout constraints.
        ///
        /// + The Folder's display_name must be distinct from all other Folder's that
        /// share its parent.
        /// + The addition of the Folder must not cause the active Folder hierarchy
        /// to exceed a height of 4. Note, the full active + deleted Folder hierarchy
        /// is allowed to reach a height of 8; this provides additional headroom when
        /// moving folders that contain deleted folders.
        /// + The addition of the Folder must not cause the total number of Folders
        /// under its parent to exceed 100.
        ///
        /// If the operation fails due to a folder constraint violation, some errors
        /// may be returned by the CreateFolder request, with status code
        /// FAILED_PRECONDITION and an error description. Other folder constraint
        /// violations will be communicated in the Operation, with the specific
        /// PreconditionFailure returned via the details list in the Operation.error
        /// field.
        ///
        /// The caller must have `resourcemanager.folders.create` permission on the
        /// identified parent.
        pub async fn create_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFolderRequest>,
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
                "/google.cloud.resourcemanager.v2.Folders/CreateFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "CreateFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Folder, changing its display_name.
        /// Changes to the folder display_name will be rejected if they violate either
        /// the display_name formatting rules or naming constraints described in
        /// the [CreateFolder][google.cloud.resourcemanager.v2.Folders.CreateFolder] documentation.
        ///
        /// The Folder's display name must start and end with a letter or digit,
        /// may contain letters, digits, spaces, hyphens and underscores and can be
        /// no longer than 30 characters. This is captured by the regular expression:
        /// [\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?.
        /// The caller must have `resourcemanager.folders.update` permission on the
        /// identified folder.
        ///
        /// If the update fails due to the unique name constraint then a
        /// PreconditionFailure explaining this violation will be returned
        /// in the Status.details field.
        pub async fn update_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::Folder>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v2.Folders/UpdateFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "UpdateFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Moves a Folder under a new resource parent.
        /// Returns an Operation which can be used to track the progress of the
        /// folder move workflow.
        /// Upon success the Operation.response field will be populated with the
        /// moved Folder.
        /// Upon failure, a FolderOperationError categorizing the failure cause will
        /// be returned - if the failure occurs synchronously then the
        /// FolderOperationError will be returned via the Status.details field
        /// and if it occurs asynchronously then the FolderOperation will be returned
        /// via the Operation.error field.
        /// In addition, the Operation.metadata field will be populated with a
        /// FolderOperation message as an aid to stateless clients.
        /// Folder moves will be rejected if they violate either the naming, height
        /// or fanout constraints described in the
        /// [CreateFolder][google.cloud.resourcemanager.v2.Folders.CreateFolder] documentation.
        /// The caller must have `resourcemanager.folders.move` permission on the
        /// folder's current and proposed new parent.
        pub async fn move_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveFolderRequest>,
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
                "/google.cloud.resourcemanager.v2.Folders/MoveFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "MoveFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Requests deletion of a Folder. The Folder is moved into the
        /// [DELETE_REQUESTED][google.cloud.resourcemanager.v2.Folder.LifecycleState.DELETE_REQUESTED] state
        /// immediately, and is deleted approximately 30 days later. This method may
        /// only be called on an empty Folder in the
        /// [ACTIVE][google.cloud.resourcemanager.v2.Folder.LifecycleState.ACTIVE] state, where a Folder is empty if
        /// it doesn't contain any Folders or Projects in the
        /// [ACTIVE][google.cloud.resourcemanager.v2.Folder.LifecycleState.ACTIVE] state.
        /// The caller must have `resourcemanager.folders.delete` permission on the
        /// identified folder.
        pub async fn delete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::Folder>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v2.Folders/DeleteFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "DeleteFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels the deletion request for a Folder. This method may only be
        /// called on a Folder in the
        /// [DELETE_REQUESTED][google.cloud.resourcemanager.v2.Folder.LifecycleState.DELETE_REQUESTED] state.
        /// In order to succeed, the Folder's parent must be in the
        /// [ACTIVE][google.cloud.resourcemanager.v2.Folder.LifecycleState.ACTIVE] state.
        /// In addition, reintroducing the folder into the tree must not violate
        /// folder naming, height and fanout constraints described in the
        /// [CreateFolder][google.cloud.resourcemanager.v2.Folders.CreateFolder] documentation.
        /// The caller must have `resourcemanager.folders.undelete` permission on the
        /// identified folder.
        pub async fn undelete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::Folder>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v2.Folders/UndeleteFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "UndeleteFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a Folder. The returned policy may be
        /// empty if no such policy or resource exists. The `resource` field should
        /// be the Folder's resource name, e.g. "folders/1234".
        /// The caller must have `resourcemanager.folders.getIamPolicy` permission
        /// on the identified folder.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v2.Folders/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the access control policy on a Folder, replacing any existing policy.
        /// The `resource` field should be the Folder's resource name, e.g.
        /// "folders/1234".
        /// The caller must have `resourcemanager.folders.setIamPolicy` permission
        /// on the identified folder.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v2.Folders/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns permissions that a caller has on the specified Folder.
        /// The `resource` field should be the Folder's resource name,
        /// e.g. "folders/1234".
        ///
        /// There are no permissions required for making this API call.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.resourcemanager.v2.Folders/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.resourcemanager.v2.Folders",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
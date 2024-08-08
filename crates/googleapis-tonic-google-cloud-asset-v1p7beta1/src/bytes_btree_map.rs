pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod asset {
            pub mod v1p7beta1 {
                include!("bytes_btree_map/google.cloud.asset.v1p7beta1.rs");
            }
        }
        pub mod orgpolicy {
            pub mod v1 {
                pub(crate) use googleapis_tonic_google_cloud_orgpolicy_v1::google::cloud::orgpolicy::v1::*;
            }
        }
    }
    pub mod iam {
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod identity {
        pub mod accesscontextmanager {
            pub mod r#type {
                pub(crate) use googleapis_tonic_google_identity_accesscontextmanager_type::google::identity::accesscontextmanager::r#type::*;
            }
            pub mod v1 {
                pub(crate) use googleapis_tonic_google_identity_accesscontextmanager_v1::google::identity::accesscontextmanager::v1::*;
            }
        }
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

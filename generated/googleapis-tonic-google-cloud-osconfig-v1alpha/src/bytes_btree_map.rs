pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod osconfig {
            pub mod v1alpha {
                include!("bytes_btree_map/google.cloud.osconfig.v1alpha.rs");
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

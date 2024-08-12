pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod workflows {
            pub mod v1beta {
                include!("bytes_hash_map/google.cloud.workflows.v1beta.rs");
            }
        }
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

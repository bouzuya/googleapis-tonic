pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod networksecurity {
            pub mod v1alpha1 {
                include!("bytes_btree_map/google.cloud.networksecurity.v1alpha1.rs");
            }
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

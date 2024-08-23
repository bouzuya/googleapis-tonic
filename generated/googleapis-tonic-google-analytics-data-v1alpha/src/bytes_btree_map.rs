pub mod google {
    pub mod analytics {
        pub mod data {
            pub mod v1alpha {
                include!("bytes_btree_map/google.analytics.data.v1alpha.rs");
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

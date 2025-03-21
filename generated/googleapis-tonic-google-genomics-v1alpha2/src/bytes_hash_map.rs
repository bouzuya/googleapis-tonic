pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod genomics {
        pub mod v1alpha2 {
            include!("bytes_hash_map/google.genomics.v1alpha2.rs");
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

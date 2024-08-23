pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod dataflow {
        pub mod v1beta3 {
            include!("bytes_hash_map/google.dataflow.v1beta3.rs");
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

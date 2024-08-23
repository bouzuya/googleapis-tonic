pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod datatransfer {
                pub mod v1 {
                    include!("bytes_hash_map/google.cloud.bigquery.datatransfer.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

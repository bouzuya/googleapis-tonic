pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod migration {
                pub mod v2 {
                    include!("vec_u8_hash_map/google.cloud.bigquery.migration.v2.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

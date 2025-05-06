pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod storage {
                pub mod v1beta {
                    include!("bytes_hash_map/google.cloud.bigquery.storage.v1beta.rs");
                }
            }
        }
    }
}

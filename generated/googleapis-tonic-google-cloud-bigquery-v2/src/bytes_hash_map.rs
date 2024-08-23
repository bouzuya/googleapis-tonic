pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod v2 {
                include!("bytes_hash_map/google.cloud.bigquery.v2.rs");
            }
        }
    }
}

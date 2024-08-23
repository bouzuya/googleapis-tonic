pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod biglake {
                pub mod v1alpha1 {
                    include!("bytes_hash_map/google.cloud.bigquery.biglake.v1alpha1.rs");
                }
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod biglake {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.bigquery.biglake.v1.rs");
                }
            }
        }
    }
}

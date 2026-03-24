pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod biglake {
            pub mod hive {
                pub mod v1beta {
                    include!("bytes_hash_map/google.cloud.biglake.hive.v1beta.rs");
                }
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod metastore {
            pub mod logging {
                pub mod v1 {
                    include!("bytes_hash_map/google.cloud.metastore.logging.v1.rs");
                }
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod dataform {
            pub mod logging {
                pub mod v1 {
                    include!("vec_u8_hash_map/google.cloud.dataform.logging.v1.rs");
                }
            }
        }
    }
}

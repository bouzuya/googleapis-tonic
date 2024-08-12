pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod security {
            pub mod publicca {
                pub mod v1 {
                    include!("vec_u8_hash_map/google.cloud.security.publicca.v1.rs");
                }
            }
        }
    }
}

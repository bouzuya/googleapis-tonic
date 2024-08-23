pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod gkeconnect {
            pub mod gateway {
                pub mod v1beta1 {
                    include!("vec_u8_hash_map/google.cloud.gkeconnect.gateway.v1beta1.rs");
                }
            }
        }
    }
}

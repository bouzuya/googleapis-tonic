pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod gkeconnect {
            pub mod gateway {
                pub mod v1 {
                    include!("vec_u8_btree_map/google.cloud.gkeconnect.gateway.v1.rs");
                }
            }
        }
    }
}

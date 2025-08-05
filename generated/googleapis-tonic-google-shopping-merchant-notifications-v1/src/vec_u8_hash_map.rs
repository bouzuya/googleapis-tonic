pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod notifications {
                pub mod v1 {
                    include!("vec_u8_hash_map/google.shopping.merchant.notifications.v1.rs");
                }
            }
        }
    }
}

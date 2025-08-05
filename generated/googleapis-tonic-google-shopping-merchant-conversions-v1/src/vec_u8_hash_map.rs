pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod conversions {
                pub mod v1 {
                    include!("vec_u8_hash_map/google.shopping.merchant.conversions.v1.rs");
                }
            }
        }
    }
}

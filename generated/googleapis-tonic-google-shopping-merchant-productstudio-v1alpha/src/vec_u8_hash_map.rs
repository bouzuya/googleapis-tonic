pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod productstudio {
                pub mod v1alpha {
                    include!("vec_u8_hash_map/google.shopping.merchant.productstudio.v1alpha.rs");
                }
            }
        }
    }
}

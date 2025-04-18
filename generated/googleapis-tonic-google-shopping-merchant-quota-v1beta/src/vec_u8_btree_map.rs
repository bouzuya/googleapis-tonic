pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod quota {
                pub mod v1beta {
                    include!("vec_u8_btree_map/google.shopping.merchant.quota.v1beta.rs");
                }
            }
        }
    }
}

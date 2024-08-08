pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod conversions {
                pub mod v1beta {
                    include!("vec_u8_btree_map/google.shopping.merchant.conversions.v1beta.rs");
                }
            }
        }
    }
}

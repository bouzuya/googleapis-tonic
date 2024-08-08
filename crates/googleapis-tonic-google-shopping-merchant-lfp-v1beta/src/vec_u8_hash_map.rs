pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod lfp {
                pub mod v1beta {
                    include!("vec_u8_hash_map/google.shopping.merchant.lfp.v1beta.rs");
                }
            }
        }
        pub mod r#type {
            pub(crate) use googleapis_tonic_google_shopping_type::google::shopping::r#type::*;
        }
    }
}

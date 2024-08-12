pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod datasources {
                pub mod v1beta {
                    include!("vec_u8_btree_map/google.shopping.merchant.datasources.v1beta.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

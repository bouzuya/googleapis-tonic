pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod datasources {
                pub mod v1beta {
                    include!("vec_u8_hash_map/google.shopping.merchant.datasources.v1beta.rs");
                }
            }
        }
        pub mod r#type {
            pub use googleapis_tonic_google_shopping_type::google::shopping::r#type::*;
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

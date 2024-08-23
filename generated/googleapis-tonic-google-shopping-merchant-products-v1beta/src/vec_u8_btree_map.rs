pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod products {
                pub mod v1beta {
                    include!("vec_u8_btree_map/google.shopping.merchant.products.v1beta.rs");
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

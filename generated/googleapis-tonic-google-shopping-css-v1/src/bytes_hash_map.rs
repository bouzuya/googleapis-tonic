pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod css {
            pub mod v1 {
                include!("bytes_hash_map/google.shopping.css.v1.rs");
            }
        }
        pub mod r#type {
            pub use googleapis_tonic_google_shopping_type::google::shopping::r#type::*;
        }
    }
}

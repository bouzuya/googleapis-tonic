pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod reports {
                pub mod v1alpha {
                    include!("bytes_hash_map/google.shopping.merchant.reports.v1alpha.rs");
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

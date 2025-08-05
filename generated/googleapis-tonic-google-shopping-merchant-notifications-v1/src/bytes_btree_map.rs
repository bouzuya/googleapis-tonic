pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod notifications {
                pub mod v1 {
                    include!("bytes_btree_map/google.shopping.merchant.notifications.v1.rs");
                }
            }
        }
    }
}

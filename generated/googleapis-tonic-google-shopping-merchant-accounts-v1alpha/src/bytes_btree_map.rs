pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod accounts {
                pub mod v1alpha {
                    include!("bytes_btree_map/google.shopping.merchant.accounts.v1alpha.rs");
                }
            }
        }
    }
}

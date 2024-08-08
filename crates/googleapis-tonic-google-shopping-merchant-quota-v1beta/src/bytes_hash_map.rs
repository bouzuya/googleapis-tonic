pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod shopping {
        pub mod merchant {
            pub mod quota {
                pub mod v1beta {
                    include!("bytes_hash_map/google.shopping.merchant.quota.v1beta.rs");
                }
            }
        }
    }
}

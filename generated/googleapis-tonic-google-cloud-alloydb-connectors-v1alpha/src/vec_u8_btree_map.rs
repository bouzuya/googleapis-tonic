pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod alloydb {
            pub mod connectors {
                pub mod v1alpha {
                    include!("vec_u8_btree_map/google.cloud.alloydb.connectors.v1alpha.rs");
                }
            }
        }
    }
}

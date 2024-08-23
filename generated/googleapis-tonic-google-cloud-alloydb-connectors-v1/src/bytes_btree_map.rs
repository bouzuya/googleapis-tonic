pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod alloydb {
            pub mod connectors {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.alloydb.connectors.v1.rs");
                }
            }
        }
    }
}

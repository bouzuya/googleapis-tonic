pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod alloydb {
            pub mod connectors {
                pub mod v1beta {
                    include!("bytes_btree_map/google.cloud.alloydb.connectors.v1beta.rs");
                }
            }
        }
    }
}

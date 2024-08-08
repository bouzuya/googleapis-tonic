pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod eventarc {
            pub mod publishing {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.eventarc.publishing.v1.rs");
                }
            }
        }
    }
}

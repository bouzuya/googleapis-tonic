pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod gkehub {
            pub mod servicemesh {
                pub mod v1beta {
                    include!("bytes_btree_map/google.cloud.gkehub.servicemesh.v1beta.rs");
                }
            }
        }
    }
}

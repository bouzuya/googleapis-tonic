pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod home {
        pub mod enterprise {
            pub mod sdm {
                pub mod v1 {
                    include!("bytes_btree_map/google.home.enterprise.sdm.v1.rs");
                }
            }
        }
    }
}
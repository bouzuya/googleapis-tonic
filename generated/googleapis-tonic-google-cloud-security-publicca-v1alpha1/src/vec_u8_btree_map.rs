pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod security {
            pub mod publicca {
                pub mod v1alpha1 {
                    include!("vec_u8_btree_map/google.cloud.security.publicca.v1alpha1.rs");
                }
            }
        }
    }
}

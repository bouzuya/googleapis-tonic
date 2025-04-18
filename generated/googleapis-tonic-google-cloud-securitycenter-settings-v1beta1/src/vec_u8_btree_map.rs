pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod securitycenter {
            pub mod settings {
                pub mod v1beta1 {
                    include!("vec_u8_btree_map/google.cloud.securitycenter.settings.v1beta1.rs");
                }
            }
        }
    }
}

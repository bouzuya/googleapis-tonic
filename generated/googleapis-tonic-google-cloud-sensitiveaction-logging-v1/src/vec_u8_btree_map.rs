pub mod google {
    pub mod cloud {
        pub mod securitycenter {
            pub mod v1 {
                pub use googleapis_tonic_google_cloud_securitycenter_v1::google::cloud::securitycenter::v1::*;
            }
        }
        pub mod sensitiveaction {
            pub mod logging {
                pub mod v1 {
                    include!("vec_u8_btree_map/google.cloud.sensitiveaction.logging.v1.rs");
                }
            }
        }
    }
}

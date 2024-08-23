pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod kms {
            pub mod inventory {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.kms.inventory.v1.rs");
                }
            }
            pub mod v1 {
                pub use googleapis_tonic_google_cloud_kms_v1::google::cloud::kms::v1::*;
            }
        }
    }
}

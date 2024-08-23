pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod oslogin {
            pub mod common {
                pub use googleapis_tonic_google_cloud_oslogin_common::google::cloud::oslogin::common::*;
            }
            pub mod v1beta {
                include!("vec_u8_btree_map/google.cloud.oslogin.v1beta.rs");
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod oslogin {
            pub mod common {
                include!("bytes_btree_map/google.cloud.oslogin.common.rs");
            }
        }
    }
}

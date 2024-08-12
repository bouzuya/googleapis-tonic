pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod common {
            include!("bytes_btree_map/google.cloud.common.rs");
        }
    }
}

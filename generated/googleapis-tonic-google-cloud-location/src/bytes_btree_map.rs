pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod location {
            include!("bytes_btree_map/google.cloud.location.rs");
        }
    }
}

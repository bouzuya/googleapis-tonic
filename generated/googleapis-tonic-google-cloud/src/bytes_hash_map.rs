pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        include!("bytes_hash_map/google.cloud.rs");
    }
}
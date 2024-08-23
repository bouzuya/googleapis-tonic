pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod location {
            include!("bytes_hash_map/google.cloud.location.rs");
        }
    }
}

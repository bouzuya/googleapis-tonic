pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod pubsub {
        pub mod v1 {
            include!("bytes_hash_map/google.pubsub.v1.rs");
        }
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod gkerecommender {
            pub mod v1 {
                include!("bytes_hash_map/google.cloud.gkerecommender.v1.rs");
            }
        }
    }
}

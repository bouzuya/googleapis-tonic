pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod integrations {
            pub mod v1alpha {
                include!("bytes_hash_map/google.cloud.integrations.v1alpha.rs");
            }
        }
    }
}

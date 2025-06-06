pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod support {
            pub mod v2beta {
                include!("bytes_hash_map/google.cloud.support.v2beta.rs");
            }
        }
    }
}

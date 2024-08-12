pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod language {
            pub mod v1beta2 {
                include!("bytes_hash_map/google.cloud.language.v1beta2.rs");
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod assistant {
        pub mod embedded {
            pub mod v1alpha2 {
                include!("bytes_hash_map/google.assistant.embedded.v1alpha2.rs");
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

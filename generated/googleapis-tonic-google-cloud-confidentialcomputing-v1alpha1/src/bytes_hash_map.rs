pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod confidentialcomputing {
            pub mod v1alpha1 {
                include!("bytes_hash_map/google.cloud.confidentialcomputing.v1alpha1.rs");
            }
        }
    }
}

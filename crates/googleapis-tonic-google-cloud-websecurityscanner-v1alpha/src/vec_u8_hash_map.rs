pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod websecurityscanner {
            pub mod v1alpha {
                include!("vec_u8_hash_map/google.cloud.websecurityscanner.v1alpha.rs");
            }
        }
    }
}
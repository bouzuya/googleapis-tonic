pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub(crate) use googleapis_tonic_google_cloud::google::cloud::*;
        pub mod compute {
            pub mod v1 {
                include!("vec_u8_hash_map/google.cloud.compute.v1.rs");
            }
        }
    }
}
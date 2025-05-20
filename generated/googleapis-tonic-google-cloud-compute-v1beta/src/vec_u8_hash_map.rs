pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub use googleapis_tonic_google_cloud::google::cloud::*;
        pub mod compute {
            pub mod v1beta {
                include!("vec_u8_hash_map/google.cloud.compute.v1beta.rs");
            }
        }
    }
}

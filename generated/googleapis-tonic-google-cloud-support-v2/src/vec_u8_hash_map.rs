pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod support {
            pub mod v2 {
                include!("vec_u8_hash_map/google.cloud.support.v2.rs");
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod binaryauthorization {
            pub mod v1 {
                include!("vec_u8_hash_map/google.cloud.binaryauthorization.v1.rs");
            }
        }
    }
}
pub mod grafeas {
    pub mod v1 {
        pub use googleapis_tonic_grafeas_v1::grafeas::v1::*;
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod spanner {
        pub mod adapter {
            pub mod v1 {
                include!("vec_u8_hash_map/google.spanner.adapter.v1.rs");
            }
        }
    }
}

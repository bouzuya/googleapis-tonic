pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod area120 {
        pub mod tables {
            pub mod v1alpha1 {
                include!("vec_u8_hash_map/google.area120.tables.v1alpha1.rs");
            }
        }
    }
}

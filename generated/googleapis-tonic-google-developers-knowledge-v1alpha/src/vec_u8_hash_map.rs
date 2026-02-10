pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod developers {
        pub mod knowledge {
            pub mod v1alpha {
                include!("vec_u8_hash_map/google.developers.knowledge.v1alpha.rs");
            }
        }
    }
}

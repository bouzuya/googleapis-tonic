pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
        pub mod cloudquotas {
            pub mod v1beta {
                include!("vec_u8_hash_map/google.api.cloudquotas.v1beta.rs");
            }
        }
    }
}

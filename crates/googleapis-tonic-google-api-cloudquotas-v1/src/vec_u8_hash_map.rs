pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
        pub mod cloudquotas {
            pub mod v1 {
                include!("vec_u8_hash_map/google.api.cloudquotas.v1.rs");
            }
        }
    }
}

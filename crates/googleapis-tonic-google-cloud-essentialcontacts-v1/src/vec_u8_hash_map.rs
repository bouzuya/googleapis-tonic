pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod essentialcontacts {
            pub mod v1 {
                include!("vec_u8_hash_map/google.cloud.essentialcontacts.v1.rs");
            }
        }
    }
}

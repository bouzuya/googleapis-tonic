pub mod google {
    pub mod ai {
        pub mod generativelanguage {
            pub mod v1 {
                include!("vec_u8_hash_map/google.ai.generativelanguage.v1.rs");
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

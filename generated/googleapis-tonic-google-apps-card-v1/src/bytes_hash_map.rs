pub mod google {
    pub mod apps {
        pub mod card {
            pub mod v1 {
                include!("bytes_hash_map/google.apps.card.v1.rs");
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

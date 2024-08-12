pub mod google {
    pub mod analytics {
        pub mod admin {
            pub mod v1beta {
                include!("vec_u8_hash_map/google.analytics.admin.v1beta.rs");
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

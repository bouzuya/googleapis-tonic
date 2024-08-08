pub mod google {
    pub mod actions {
        pub mod sdk {
            pub mod v2 {
                pub mod interactionmodel {
                    pub mod r#type {
                        include!("vec_u8_hash_map/google.actions.sdk.v2.interactionmodel.r#type.rs");
                    }
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

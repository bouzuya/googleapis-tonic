pub mod google {
    pub mod actions {
        pub mod sdk {
            pub mod v2 {
                pub mod interactionmodel {
                    pub mod prompt {
                        include!("vec_u8_hash_map/google.actions.sdk.v2.interactionmodel.prompt.rs");
                    }
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

pub mod google {
    pub mod actions {
        pub mod sdk {
            pub mod v2 {
                pub mod interactionmodel {
                    pub mod prompt {
                        pub use googleapis_tonic_google_actions_sdk_v2_interactionmodel_prompt::google::actions::sdk::v2::interactionmodel::prompt::*;
                    }
                    pub mod r#type {
                        pub use googleapis_tonic_google_actions_sdk_v2_interactionmodel_type::google::actions::sdk::v2::interactionmodel::r#type::*;
                    }
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

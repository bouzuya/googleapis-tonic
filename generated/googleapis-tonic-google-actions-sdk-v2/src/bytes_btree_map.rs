pub mod google {
    pub mod actions {
        pub mod sdk {
            pub mod v2 {
                pub mod conversation {
                    pub use googleapis_tonic_google_actions_sdk_v2_conversation::google::actions::sdk::v2::conversation::*;
                }
                pub mod interactionmodel {
                    pub use googleapis_tonic_google_actions_sdk_v2_interactionmodel::google::actions::sdk::v2::interactionmodel::*;
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
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

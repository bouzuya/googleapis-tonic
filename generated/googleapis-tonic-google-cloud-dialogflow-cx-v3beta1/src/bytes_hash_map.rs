pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod dialogflow {
            pub mod cx {
                pub mod v3beta1 {
                    include!("bytes_hash_map/google.cloud.dialogflow.cx.v3beta1.rs");
                }
            }
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

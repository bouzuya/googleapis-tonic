pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod workflows {
            pub mod executions {
                pub mod v1beta {
                    include!("vec_u8_hash_map/google.cloud.workflows.executions.v1beta.rs");
                }
            }
        }
    }
}

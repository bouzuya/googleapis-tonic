pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod osconfig {
            pub mod agentendpoint {
                pub mod v1beta {
                    include!("bytes_hash_map/google.cloud.osconfig.agentendpoint.v1beta.rs");
                }
            }
        }
    }
}
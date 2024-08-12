pub mod google {
    pub mod cloud {
        pub mod aiplatform {
            pub mod logging {
                include!("vec_u8_hash_map/google.cloud.aiplatform.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

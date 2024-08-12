pub mod google {
    pub mod cloud {
        pub mod aiplatform {
            pub mod logging {
                include!("bytes_btree_map/google.cloud.aiplatform.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

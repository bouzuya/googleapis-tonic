pub mod google {
    pub mod cloud {
        pub mod healthcare {
            pub mod logging {
                include!("bytes_hash_map/google.cloud.healthcare.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

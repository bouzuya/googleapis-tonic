pub mod google {
    pub mod cloud {
        pub mod identitytoolkit {
            pub mod logging {
                include!("bytes_hash_map/google.cloud.identitytoolkit.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

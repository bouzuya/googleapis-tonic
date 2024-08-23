pub mod google {
    pub mod cloud {
        pub mod gsuiteaddons {
            pub mod logging {
                pub mod v1 {
                    include!("bytes_hash_map/google.cloud.gsuiteaddons.logging.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

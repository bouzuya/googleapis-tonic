pub mod google {
    pub mod cloud {
        pub mod healthcare {
            pub mod logging {
                include!("vec_u8_hash_map/google.cloud.healthcare.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

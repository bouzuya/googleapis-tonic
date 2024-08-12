pub mod google {
    pub mod cloud {
        pub mod retail {
            pub mod logging {
                include!("vec_u8_hash_map/google.cloud.retail.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

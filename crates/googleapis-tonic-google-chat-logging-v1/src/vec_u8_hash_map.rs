pub mod google {
    pub mod chat {
        pub mod logging {
            pub mod v1 {
                include!("vec_u8_hash_map/google.chat.logging.v1.rs");
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}
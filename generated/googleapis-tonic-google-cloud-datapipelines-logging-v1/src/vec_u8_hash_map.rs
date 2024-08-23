pub mod google {
    pub mod cloud {
        pub mod datapipelines {
            pub mod logging {
                pub mod v1 {
                    include!("vec_u8_hash_map/google.cloud.datapipelines.logging.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

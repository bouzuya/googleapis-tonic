pub mod google {
    pub mod cloud {
        pub mod kms {
            pub mod logging {
                pub mod v1 {
                    include!("vec_u8_btree_map/google.cloud.kms.logging.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

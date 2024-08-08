pub mod google {
    pub mod cloud {
        pub mod gkebackup {
            pub mod logging {
                pub mod v1 {
                    include!("vec_u8_btree_map/google.cloud.gkebackup.logging.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

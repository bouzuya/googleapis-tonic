pub mod google {
    pub mod cloud {
        pub mod retail {
            pub mod logging {
                include!("vec_u8_btree_map/google.cloud.retail.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

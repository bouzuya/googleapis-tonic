pub mod google {
    pub mod cloud {
        pub mod discoveryengine {
            pub mod logging {
                include!("bytes_btree_map/google.cloud.discoveryengine.logging.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

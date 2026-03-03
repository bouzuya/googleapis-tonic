pub mod google {
    pub mod devtools {
        pub mod artifactregistry {
            pub mod logging {
                pub mod v1 {
                    include!("bytes_btree_map/google.devtools.artifactregistry.logging.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

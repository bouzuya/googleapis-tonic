pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod assistant {
        pub mod embedded {
            pub mod v1alpha1 {
                include!("bytes_btree_map/google.assistant.embedded.v1alpha1.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

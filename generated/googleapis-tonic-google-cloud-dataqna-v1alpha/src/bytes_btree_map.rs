pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod dataqna {
            pub mod v1alpha {
                include!("bytes_btree_map/google.cloud.dataqna.v1alpha.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
        pub mod apikeys {
            pub mod v2 {
                include!("bytes_btree_map/google.api.apikeys.v2.rs");
            }
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

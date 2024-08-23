pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod storage {
        pub mod control {
            pub mod v2 {
                include!("bytes_btree_map/google.storage.control.v2.rs");
            }
        }
    }
}

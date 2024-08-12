pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod storage {
        pub mod control {
            pub mod v2 {
                include!("vec_u8_hash_map/google.storage.control.v2.rs");
            }
        }
    }
}

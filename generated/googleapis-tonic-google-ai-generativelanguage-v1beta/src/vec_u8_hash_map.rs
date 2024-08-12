pub mod google {
    pub mod ai {
        pub mod generativelanguage {
            pub mod v1beta {
                include!("vec_u8_hash_map/google.ai.generativelanguage.v1beta.rs");
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod firestore {
        pub mod admin {
            pub mod v1beta2 {
                include!("vec_u8_btree_map/google.firestore.admin.v1beta2.rs");
            }
        }
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

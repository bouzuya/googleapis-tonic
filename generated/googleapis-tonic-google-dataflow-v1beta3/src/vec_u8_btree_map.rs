pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod dataflow {
        pub mod v1beta3 {
            include!("vec_u8_btree_map/google.dataflow.v1beta3.rs");
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}
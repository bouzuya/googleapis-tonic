pub mod google {
    pub mod analytics {
        pub mod data {
            pub mod v1beta {
                include!("vec_u8_btree_map/google.analytics.data.v1beta.rs");
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

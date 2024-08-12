pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod monitoring {
        pub mod metricsscope {
            pub mod v1 {
                include!("bytes_btree_map/google.monitoring.metricsscope.v1.rs");
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

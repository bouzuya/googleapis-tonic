pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod commerce {
            pub mod consumer {
                pub mod procurement {
                    pub mod v1alpha1 {
                        include!("bytes_hash_map/google.cloud.commerce.consumer.procurement.v1alpha1.rs");
                    }
                }
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
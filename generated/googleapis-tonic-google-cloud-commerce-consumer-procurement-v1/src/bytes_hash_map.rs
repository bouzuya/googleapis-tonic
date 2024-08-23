pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod commerce {
            pub mod consumer {
                pub mod procurement {
                    pub mod v1 {
                        include!("bytes_hash_map/google.cloud.commerce.consumer.procurement.v1.rs");
                    }
                }
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

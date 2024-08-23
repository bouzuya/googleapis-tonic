pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod paymentgateway {
            pub mod issuerswitch {
                pub mod v1 {
                    include!("vec_u8_btree_map/google.cloud.paymentgateway.issuerswitch.v1.rs");
                }
            }
        }
    }
    pub mod logging {
        pub mod r#type {
            pub use googleapis_tonic_google_logging_type::google::logging::r#type::*;
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod paymentgateway {
            pub mod issuerswitch {
                pub mod accountmanager {
                    pub mod v1 {
                        include!("bytes_hash_map/google.cloud.paymentgateway.issuerswitch.accountmanager.v1.rs");
                    }
                }
                pub mod v1 {
                    pub(crate) use googleapis_tonic_google_cloud_paymentgateway_issuerswitch_v1::google::cloud::paymentgateway::issuerswitch::v1::*;
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
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

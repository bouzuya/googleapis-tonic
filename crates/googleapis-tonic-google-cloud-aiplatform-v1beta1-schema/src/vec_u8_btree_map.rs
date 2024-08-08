pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod aiplatform {
            pub mod v1beta1 {
                pub mod schema {
                    pub mod predict {
                        pub mod instance {
                            pub(crate) use googleapis_tonic_google_cloud_aiplatform_v1beta1_schema_predict_instance::google::cloud::aiplatform::v1beta1::schema::predict::instance::*;
                        }
                    }
                }
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

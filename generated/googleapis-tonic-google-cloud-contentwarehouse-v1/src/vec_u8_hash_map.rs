pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod contentwarehouse {
            pub mod v1 {
                include!("vec_u8_hash_map/google.cloud.contentwarehouse.v1.rs");
            }
        }
        pub mod documentai {
            pub mod v1 {
                pub use googleapis_tonic_google_cloud_documentai_v1::google::cloud::documentai::v1::*;
            }
        }
    }
    pub mod iam {
        pub mod v1 {
            pub use googleapis_tonic_google_iam_v1::google::iam::v1::*;
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

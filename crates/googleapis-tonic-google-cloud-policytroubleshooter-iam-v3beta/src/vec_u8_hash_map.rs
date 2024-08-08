pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod policytroubleshooter {
            pub mod iam {
                pub mod v3beta {
                    include!("vec_u8_hash_map/google.cloud.policytroubleshooter.iam.v3beta.rs");
                }
            }
        }
    }
    pub mod iam {
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
        pub mod v2 {
            pub(crate) use googleapis_tonic_google_iam_v2::google::iam::v2::*;
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

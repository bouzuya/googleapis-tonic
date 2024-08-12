pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod policytroubleshooter {
            pub mod v1 {
                include!("bytes_btree_map/google.cloud.policytroubleshooter.v1.rs");
            }
        }
    }
    pub mod iam {
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

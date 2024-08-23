pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod common {
            pub use googleapis_tonic_google_cloud_common::google::cloud::common::*;
        }
        pub mod filestore {
            pub mod v1beta1 {
                include!("vec_u8_btree_map/google.cloud.filestore.v1beta1.rs");
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

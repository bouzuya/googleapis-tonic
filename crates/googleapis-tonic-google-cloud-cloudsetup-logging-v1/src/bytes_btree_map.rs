pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod cloudsetup {
            pub mod logging {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.cloudsetup.logging.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

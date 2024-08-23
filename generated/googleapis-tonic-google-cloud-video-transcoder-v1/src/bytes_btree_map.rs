pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod video {
            pub mod transcoder {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.video.transcoder.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

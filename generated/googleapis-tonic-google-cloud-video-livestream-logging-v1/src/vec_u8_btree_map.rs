pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod video {
            pub mod livestream {
                pub mod logging {
                    pub mod v1 {
                        include!("vec_u8_btree_map/google.cloud.video.livestream.logging.v1.rs");
                    }
                }
                pub mod v1 {
                    pub(crate) use googleapis_tonic_google_cloud_video_livestream_v1::google::cloud::video::livestream::v1::*;
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
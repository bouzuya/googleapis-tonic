pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod recommender {
            pub mod logging {
                pub mod v1 {
                    include!("vec_u8_btree_map/google.cloud.recommender.logging.v1.rs");
                }
            }
            pub mod v1 {
                pub use googleapis_tonic_google_cloud_recommender_v1::google::cloud::recommender::v1::*;
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

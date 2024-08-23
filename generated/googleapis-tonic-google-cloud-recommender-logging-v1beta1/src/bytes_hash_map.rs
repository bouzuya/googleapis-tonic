pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod recommender {
            pub mod logging {
                pub mod v1beta1 {
                    include!("bytes_hash_map/google.cloud.recommender.logging.v1beta1.rs");
                }
            }
            pub mod v1beta1 {
                pub use googleapis_tonic_google_cloud_recommender_v1beta1::google::cloud::recommender::v1beta1::*;
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

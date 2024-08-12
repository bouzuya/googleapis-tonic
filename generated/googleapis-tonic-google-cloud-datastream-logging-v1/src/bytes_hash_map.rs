pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod datastream {
            pub mod logging {
                pub mod v1 {
                    include!("bytes_hash_map/google.cloud.datastream.logging.v1.rs");
                }
            }
            pub mod v1 {
                pub(crate) use googleapis_tonic_google_cloud_datastream_v1::google::cloud::datastream::v1::*;
            }
        }
    }
}

pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v17 {
                pub mod common {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v17_common::google::ads::googleads::v17::common::*;
                }
                pub mod enums {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v17_enums::google::ads::googleads::v17::enums::*;
                }
                pub mod errors {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v17_errors::google::ads::googleads::v17::errors::*;
                }
                pub mod resources {
                    include!("bytes_hash_map/google.ads.googleads.v17.resources.rs");
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

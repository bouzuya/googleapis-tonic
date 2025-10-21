pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v22 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v22_common::google::ads::googleads::v22::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v22_enums::google::ads::googleads::v22::enums::*;
                }
                pub mod errors {
                    pub use googleapis_tonic_google_ads_googleads_v22_errors::google::ads::googleads::v22::errors::*;
                }
                pub mod resources {
                    include!("bytes_hash_map/google.ads.googleads.v22.resources.rs");
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

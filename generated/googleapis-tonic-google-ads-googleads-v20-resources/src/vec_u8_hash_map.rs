pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v20 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v20_common::google::ads::googleads::v20::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v20_enums::google::ads::googleads::v20::enums::*;
                }
                pub mod errors {
                    pub use googleapis_tonic_google_ads_googleads_v20_errors::google::ads::googleads::v20::errors::*;
                }
                pub mod resources {
                    include!("vec_u8_hash_map/google.ads.googleads.v20.resources.rs");
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

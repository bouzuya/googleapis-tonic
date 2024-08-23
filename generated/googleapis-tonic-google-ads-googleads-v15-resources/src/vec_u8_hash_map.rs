pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v15 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v15_common::google::ads::googleads::v15::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v15_enums::google::ads::googleads::v15::enums::*;
                }
                pub mod errors {
                    pub use googleapis_tonic_google_ads_googleads_v15_errors::google::ads::googleads::v15::errors::*;
                }
                pub mod resources {
                    include!("vec_u8_hash_map/google.ads.googleads.v15.resources.rs");
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

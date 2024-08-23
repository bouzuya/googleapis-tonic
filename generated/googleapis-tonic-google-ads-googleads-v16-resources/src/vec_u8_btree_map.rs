pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v16 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v16_common::google::ads::googleads::v16::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v16_enums::google::ads::googleads::v16::enums::*;
                }
                pub mod errors {
                    pub use googleapis_tonic_google_ads_googleads_v16_errors::google::ads::googleads::v16::errors::*;
                }
                pub mod resources {
                    include!("vec_u8_btree_map/google.ads.googleads.v16.resources.rs");
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

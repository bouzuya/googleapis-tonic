pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v19 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v19_common::google::ads::googleads::v19::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v19_enums::google::ads::googleads::v19::enums::*;
                }
                pub mod errors {
                    pub use googleapis_tonic_google_ads_googleads_v19_errors::google::ads::googleads::v19::errors::*;
                }
                pub mod resources {
                    include!("vec_u8_btree_map/google.ads.googleads.v19.resources.rs");
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

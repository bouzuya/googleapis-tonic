pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v20 {
                pub mod common {
                    include!("vec_u8_hash_map/google.ads.googleads.v20.common.rs");
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v20_enums::google::ads::googleads::v20::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

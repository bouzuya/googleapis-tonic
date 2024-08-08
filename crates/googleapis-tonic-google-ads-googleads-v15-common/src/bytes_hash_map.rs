pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v15 {
                pub mod common {
                    include!("bytes_hash_map/google.ads.googleads.v15.common.rs");
                }
                pub mod enums {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v15_enums::google::ads::googleads::v15::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

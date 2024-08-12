pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v16 {
                pub mod common {
                    include!("vec_u8_btree_map/google.ads.googleads.v16.common.rs");
                }
                pub mod enums {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v16_enums::google::ads::googleads::v16::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

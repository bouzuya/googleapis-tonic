pub mod google {
    pub mod ads {
        pub mod searchads360 {
            pub mod v0 {
                pub mod common {
                    include!("vec_u8_hash_map/google.ads.searchads360.v0.common.rs");
                }
                pub mod enums {
                    pub(crate) use googleapis_tonic_google_ads_searchads360_v0_enums::google::ads::searchads360::v0::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}
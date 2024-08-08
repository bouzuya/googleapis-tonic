pub mod google {
    pub mod ads {
        pub mod searchads360 {
            pub mod v0 {
                pub mod common {
                    pub(crate) use googleapis_tonic_google_ads_searchads360_v0_common::google::ads::searchads360::v0::common::*;
                }
                pub mod enums {
                    pub(crate) use googleapis_tonic_google_ads_searchads360_v0_enums::google::ads::searchads360::v0::enums::*;
                }
                pub mod resources {
                    pub(crate) use googleapis_tonic_google_ads_searchads360_v0_resources::google::ads::searchads360::v0::resources::*;
                }
                pub mod services {
                    include!("vec_u8_hash_map/google.ads.searchads360.v0.services.rs");
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

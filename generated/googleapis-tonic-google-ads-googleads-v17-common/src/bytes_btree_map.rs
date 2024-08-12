pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v17 {
                pub mod common {
                    include!("bytes_btree_map/google.ads.googleads.v17.common.rs");
                }
                pub mod enums {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v17_enums::google::ads::googleads::v17::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
}

pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v15 {
                pub mod common {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v15_common::google::ads::googleads::v15::common::*;
                }
                pub mod enums {
                    pub(crate) use googleapis_tonic_google_ads_googleads_v15_enums::google::ads::googleads::v15::enums::*;
                }
                pub mod errors {
                    include!("bytes_btree_map/google.ads.googleads.v15.errors.rs");
                }
            }
        }
    }
}

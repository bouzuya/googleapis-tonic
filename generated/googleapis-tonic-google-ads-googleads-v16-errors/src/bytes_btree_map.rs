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
                    include!("bytes_btree_map/google.ads.googleads.v16.errors.rs");
                }
            }
        }
    }
}

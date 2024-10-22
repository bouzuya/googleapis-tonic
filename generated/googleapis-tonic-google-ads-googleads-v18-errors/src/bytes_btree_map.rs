pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v18 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v18_common::google::ads::googleads::v18::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v18_enums::google::ads::googleads::v18::enums::*;
                }
                pub mod errors {
                    include!("bytes_btree_map/google.ads.googleads.v18.errors.rs");
                }
            }
        }
    }
}

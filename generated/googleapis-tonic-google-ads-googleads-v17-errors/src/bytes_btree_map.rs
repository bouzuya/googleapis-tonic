pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v17 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v17_common::google::ads::googleads::v17::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v17_enums::google::ads::googleads::v17::enums::*;
                }
                pub mod errors {
                    include!("bytes_btree_map/google.ads.googleads.v17.errors.rs");
                }
            }
        }
    }
}

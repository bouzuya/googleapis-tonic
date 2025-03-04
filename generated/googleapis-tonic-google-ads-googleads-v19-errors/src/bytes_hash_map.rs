pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v19 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v19_common::google::ads::googleads::v19::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v19_enums::google::ads::googleads::v19::enums::*;
                }
                pub mod errors {
                    include!("bytes_hash_map/google.ads.googleads.v19.errors.rs");
                }
            }
        }
    }
}

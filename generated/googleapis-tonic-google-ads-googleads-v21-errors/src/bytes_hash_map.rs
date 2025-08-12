pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v21 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v21_common::google::ads::googleads::v21::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v21_enums::google::ads::googleads::v21::enums::*;
                }
                pub mod errors {
                    include!("bytes_hash_map/google.ads.googleads.v21.errors.rs");
                }
            }
        }
    }
}

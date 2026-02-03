pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v23 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v23_common::google::ads::googleads::v23::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v23_enums::google::ads::googleads::v23::enums::*;
                }
                pub mod errors {
                    include!("bytes_hash_map/google.ads.googleads.v23.errors.rs");
                }
            }
        }
    }
}

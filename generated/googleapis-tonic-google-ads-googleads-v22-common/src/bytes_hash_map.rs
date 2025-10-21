pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v22 {
                pub mod common {
                    include!("bytes_hash_map/google.ads.googleads.v22.common.rs");
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v22_enums::google::ads::googleads::v22::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

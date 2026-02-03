pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v23 {
                pub mod common {
                    include!("bytes_hash_map/google.ads.googleads.v23.common.rs");
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v23_enums::google::ads::googleads::v23::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

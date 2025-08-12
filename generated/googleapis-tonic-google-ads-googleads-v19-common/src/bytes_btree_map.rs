pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v19 {
                pub mod common {
                    include!("bytes_btree_map/google.ads.googleads.v19.common.rs");
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v19_enums::google::ads::googleads::v19::enums::*;
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

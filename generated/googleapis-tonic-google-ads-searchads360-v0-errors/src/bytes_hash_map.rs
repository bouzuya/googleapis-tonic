pub mod google {
    pub mod ads {
        pub mod searchads360 {
            pub mod v0 {
                pub mod common {
                    pub(crate) use googleapis_tonic_google_ads_searchads360_v0_common::google::ads::searchads360::v0::common::*;
                }
                pub mod errors {
                    include!("bytes_hash_map/google.ads.searchads360.v0.errors.rs");
                }
            }
        }
    }
}

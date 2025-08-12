pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v20 {
                pub mod common {
                    pub use googleapis_tonic_google_ads_googleads_v20_common::google::ads::googleads::v20::common::*;
                }
                pub mod enums {
                    pub use googleapis_tonic_google_ads_googleads_v20_enums::google::ads::googleads::v20::enums::*;
                }
                pub mod errors {
                    pub use googleapis_tonic_google_ads_googleads_v20_errors::google::ads::googleads::v20::errors::*;
                }
                pub mod resources {
                    pub use googleapis_tonic_google_ads_googleads_v20_resources::google::ads::googleads::v20::resources::*;
                }
                pub mod services {
                    include!("bytes_btree_map/google.ads.googleads.v20.services.rs");
                }
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

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
                    pub use googleapis_tonic_google_ads_googleads_v21_errors::google::ads::googleads::v21::errors::*;
                }
                pub mod resources {
                    pub use googleapis_tonic_google_ads_googleads_v21_resources::google::ads::googleads::v21::resources::*;
                }
                pub mod services {
                    include!("bytes_btree_map/google.ads.googleads.v21.services.rs");
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

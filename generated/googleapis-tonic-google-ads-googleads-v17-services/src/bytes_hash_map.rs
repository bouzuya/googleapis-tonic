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
                    pub use googleapis_tonic_google_ads_googleads_v17_errors::google::ads::googleads::v17::errors::*;
                }
                pub mod resources {
                    pub use googleapis_tonic_google_ads_googleads_v17_resources::google::ads::googleads::v17::resources::*;
                }
                pub mod services {
                    include!("bytes_hash_map/google.ads.googleads.v17.services.rs");
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

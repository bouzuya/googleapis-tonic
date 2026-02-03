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
                    pub use googleapis_tonic_google_ads_googleads_v23_errors::google::ads::googleads::v23::errors::*;
                }
                pub mod resources {
                    pub use googleapis_tonic_google_ads_googleads_v23_resources::google::ads::googleads::v23::resources::*;
                }
                pub mod services {
                    include!("bytes_hash_map/google.ads.googleads.v23.services.rs");
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
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

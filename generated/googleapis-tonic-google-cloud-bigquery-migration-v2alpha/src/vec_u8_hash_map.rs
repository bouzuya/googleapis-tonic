pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod migration {
                pub mod v2alpha {
                    include!("vec_u8_hash_map/google.cloud.bigquery.migration.v2alpha.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}
pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod reservation {
                pub mod v1 {
                    include!("vec_u8_btree_map/google.cloud.bigquery.reservation.v1.rs");
                }
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

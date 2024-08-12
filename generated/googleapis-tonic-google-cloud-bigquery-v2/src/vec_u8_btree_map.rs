pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod v2 {
                include!("vec_u8_btree_map/google.cloud.bigquery.v2.rs");
            }
        }
    }
}

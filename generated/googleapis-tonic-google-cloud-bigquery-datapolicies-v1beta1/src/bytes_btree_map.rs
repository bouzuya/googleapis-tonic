pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod bigquery {
            pub mod datapolicies {
                pub mod v1beta1 {
                    include!("bytes_btree_map/google.cloud.bigquery.datapolicies.v1beta1.rs");
                }
            }
        }
    }
    pub mod iam {
        pub mod v1 {
            pub use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

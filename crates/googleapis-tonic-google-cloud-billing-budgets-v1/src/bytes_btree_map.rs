pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod billing {
            pub mod budgets {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.billing.budgets.v1.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

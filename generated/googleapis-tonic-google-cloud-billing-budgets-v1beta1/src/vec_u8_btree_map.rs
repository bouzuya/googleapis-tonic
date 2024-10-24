pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod billing {
            pub mod budgets {
                pub mod v1beta1 {
                    include!("vec_u8_btree_map/google.cloud.billing.budgets.v1beta1.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

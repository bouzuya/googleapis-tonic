pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod marketingplatform {
        pub mod admin {
            pub mod v1alpha {
                include!("bytes_btree_map/google.marketingplatform.admin.v1alpha.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

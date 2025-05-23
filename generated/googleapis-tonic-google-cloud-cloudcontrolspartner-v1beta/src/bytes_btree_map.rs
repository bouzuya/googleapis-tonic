pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod cloudcontrolspartner {
            pub mod v1beta {
                include!("bytes_btree_map/google.cloud.cloudcontrolspartner.v1beta.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

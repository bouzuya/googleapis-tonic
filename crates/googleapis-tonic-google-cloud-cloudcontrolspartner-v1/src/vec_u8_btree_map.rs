pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod cloudcontrolspartner {
            pub mod v1 {
                include!("vec_u8_btree_map/google.cloud.cloudcontrolspartner.v1.rs");
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

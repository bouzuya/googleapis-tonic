pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        include!("vec_u8_btree_map/google.cloud.rs");
    }
}

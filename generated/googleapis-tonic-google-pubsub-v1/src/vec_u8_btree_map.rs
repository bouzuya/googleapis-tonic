pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod pubsub {
        pub mod v1 {
            include!("vec_u8_btree_map/google.pubsub.v1.rs");
        }
    }
}
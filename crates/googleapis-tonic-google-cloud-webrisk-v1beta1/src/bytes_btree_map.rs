pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod webrisk {
            pub mod v1beta1 {
                include!("bytes_btree_map/google.cloud.webrisk.v1beta1.rs");
            }
        }
    }
}
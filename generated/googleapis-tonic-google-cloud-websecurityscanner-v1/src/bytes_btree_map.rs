pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod websecurityscanner {
            pub mod v1 {
                include!("bytes_btree_map/google.cloud.websecurityscanner.v1.rs");
            }
        }
    }
}

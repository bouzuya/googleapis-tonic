pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod iam {
        pub mod credentials {
            pub mod v1 {
                include!("bytes_btree_map/google.iam.credentials.v1.rs");
            }
        }
    }
}

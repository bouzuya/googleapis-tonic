pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod websecurityscanner {
            pub mod v1alpha {
                include!("bytes_btree_map/google.cloud.websecurityscanner.v1alpha.rs");
            }
        }
    }
}

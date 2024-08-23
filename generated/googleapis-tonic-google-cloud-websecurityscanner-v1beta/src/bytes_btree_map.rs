pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod websecurityscanner {
            pub mod v1beta {
                include!("bytes_btree_map/google.cloud.websecurityscanner.v1beta.rs");
            }
        }
    }
}

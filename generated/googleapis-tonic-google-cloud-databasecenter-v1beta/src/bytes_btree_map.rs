pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod databasecenter {
            pub mod v1beta {
                include!("bytes_btree_map/google.cloud.databasecenter.v1beta.rs");
            }
        }
    }
}

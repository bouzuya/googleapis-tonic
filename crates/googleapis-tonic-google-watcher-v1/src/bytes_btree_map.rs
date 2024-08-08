pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod watcher {
        pub mod v1 {
            include!("bytes_btree_map/google.watcher.v1.rs");
        }
    }
}

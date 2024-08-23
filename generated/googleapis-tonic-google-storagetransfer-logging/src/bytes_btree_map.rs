pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod storagetransfer {
        pub mod logging {
            include!("bytes_btree_map/google.storagetransfer.logging.rs");
        }
    }
}

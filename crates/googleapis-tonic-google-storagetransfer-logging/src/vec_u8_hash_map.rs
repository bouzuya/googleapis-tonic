pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod storagetransfer {
        pub mod logging {
            include!("vec_u8_hash_map/google.storagetransfer.logging.rs");
        }
    }
}
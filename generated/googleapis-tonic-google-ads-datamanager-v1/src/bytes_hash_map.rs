pub mod google {
    pub mod ads {
        pub mod datamanager {
            pub mod v1 {
                include!("bytes_hash_map/google.ads.datamanager.v1.rs");
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

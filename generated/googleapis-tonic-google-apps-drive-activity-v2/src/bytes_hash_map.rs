pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod drive {
            pub mod activity {
                pub mod v2 {
                    include!("bytes_hash_map/google.apps.drive.activity.v2.rs");
                }
            }
        }
    }
}

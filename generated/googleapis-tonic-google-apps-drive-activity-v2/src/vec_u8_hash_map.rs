pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod drive {
            pub mod activity {
                pub mod v2 {
                    include!("vec_u8_hash_map/google.apps.drive.activity.v2.rs");
                }
            }
        }
    }
}

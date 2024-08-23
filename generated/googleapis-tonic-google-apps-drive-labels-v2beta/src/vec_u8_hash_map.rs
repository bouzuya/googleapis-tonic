pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod drive {
            pub mod labels {
                pub mod v2beta {
                    include!("vec_u8_hash_map/google.apps.drive.labels.v2beta.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

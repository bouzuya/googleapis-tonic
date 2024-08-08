pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod drive {
            pub mod labels {
                pub mod v2beta {
                    include!("bytes_btree_map/google.apps.drive.labels.v2beta.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod maps {
        pub mod areainsights {
            pub mod v1 {
                include!("bytes_btree_map/google.maps.areainsights.v1.rs");
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

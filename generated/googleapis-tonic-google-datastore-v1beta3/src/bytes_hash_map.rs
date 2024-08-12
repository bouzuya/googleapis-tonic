pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod datastore {
        pub mod v1beta3 {
            include!("bytes_hash_map/google.datastore.v1beta3.rs");
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

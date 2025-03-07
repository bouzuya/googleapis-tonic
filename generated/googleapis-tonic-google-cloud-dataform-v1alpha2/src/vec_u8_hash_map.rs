pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod dataform {
            pub mod v1alpha2 {
                include!("vec_u8_hash_map/google.cloud.dataform.v1alpha2.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

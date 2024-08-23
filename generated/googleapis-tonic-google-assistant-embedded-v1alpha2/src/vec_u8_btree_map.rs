pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod assistant {
        pub mod embedded {
            pub mod v1alpha2 {
                include!("vec_u8_btree_map/google.assistant.embedded.v1alpha2.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

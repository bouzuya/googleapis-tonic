pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod language {
            pub mod v1beta1 {
                include!("vec_u8_btree_map/google.cloud.language.v1beta1.rs");
            }
        }
    }
}

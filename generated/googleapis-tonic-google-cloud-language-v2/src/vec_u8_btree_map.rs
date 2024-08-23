pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod language {
            pub mod v2 {
                include!("vec_u8_btree_map/google.cloud.language.v2.rs");
            }
        }
    }
}

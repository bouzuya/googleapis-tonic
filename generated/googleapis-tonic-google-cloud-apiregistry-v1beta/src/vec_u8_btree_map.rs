pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod apiregistry {
            pub mod v1beta {
                include!("vec_u8_btree_map/google.cloud.apiregistry.v1beta.rs");
            }
        }
    }
}

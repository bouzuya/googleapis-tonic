pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod mediatranslation {
            pub mod v1alpha1 {
                include!("vec_u8_btree_map/google.cloud.mediatranslation.v1alpha1.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod mediatranslation {
            pub mod v1beta1 {
                include!("bytes_hash_map/google.cloud.mediatranslation.v1beta1.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

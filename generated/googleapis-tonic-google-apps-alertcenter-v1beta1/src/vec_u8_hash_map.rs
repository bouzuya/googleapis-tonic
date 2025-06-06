pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod alertcenter {
            pub mod v1beta1 {
                include!("vec_u8_hash_map/google.apps.alertcenter.v1beta1.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

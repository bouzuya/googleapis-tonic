pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod maps {
        pub mod mapsplatformdatasets {
            pub mod v1 {
                include!("vec_u8_hash_map/google.maps.mapsplatformdatasets.v1.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

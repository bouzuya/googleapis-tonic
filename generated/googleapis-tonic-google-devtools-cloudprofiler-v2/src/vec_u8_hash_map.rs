pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod devtools {
        pub mod cloudprofiler {
            pub mod v2 {
                include!("vec_u8_hash_map/google.devtools.cloudprofiler.v2.rs");
            }
        }
    }
}

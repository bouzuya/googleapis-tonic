pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod devtools {
        pub mod cloudtrace {
            pub mod v1 {
                include!("vec_u8_btree_map/google.devtools.cloudtrace.v1.rs");
            }
        }
    }
}

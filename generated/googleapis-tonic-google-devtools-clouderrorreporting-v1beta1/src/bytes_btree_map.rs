pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod devtools {
        pub mod clouderrorreporting {
            pub mod v1beta1 {
                include!("bytes_btree_map/google.devtools.clouderrorreporting.v1beta1.rs");
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod devtools {
        pub mod resultstore {
            pub mod v2 {
                include!("bytes_btree_map/google.devtools.resultstore.v2.rs");
            }
        }
    }
}

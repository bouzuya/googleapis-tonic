pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod example {
        pub mod library {
            pub mod v1 {
                include!("bytes_btree_map/google.example.library.v1.rs");
            }
        }
    }
}

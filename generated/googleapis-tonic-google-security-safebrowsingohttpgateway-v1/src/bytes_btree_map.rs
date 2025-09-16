pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod security {
        pub mod safebrowsingohttpgateway {
            pub mod v1 {
                include!("bytes_btree_map/google.security.safebrowsingohttpgateway.v1.rs");
            }
        }
    }
}

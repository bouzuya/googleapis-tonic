pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod security {
        pub mod safebrowsing {
            pub mod v5alpha1 {
                include!("vec_u8_btree_map/google.security.safebrowsing.v5alpha1.rs");
            }
        }
    }
}

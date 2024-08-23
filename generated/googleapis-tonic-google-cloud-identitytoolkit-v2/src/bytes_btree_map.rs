pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod identitytoolkit {
            pub mod v2 {
                include!("bytes_btree_map/google.cloud.identitytoolkit.v2.rs");
            }
        }
    }
}

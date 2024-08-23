pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod meet {
            pub mod v2beta {
                include!("bytes_btree_map/google.apps.meet.v2beta.rs");
            }
        }
    }
}

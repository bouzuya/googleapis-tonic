pub mod google {
    pub mod ai {
        pub mod generativelanguage {
            pub mod v1 {
                include!("bytes_btree_map/google.ai.generativelanguage.v1.rs");
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

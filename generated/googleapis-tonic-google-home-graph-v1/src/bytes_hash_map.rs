pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod home {
        pub mod graph {
            pub mod v1 {
                include!("bytes_hash_map/google.home.graph.v1.rs");
            }
        }
    }
}

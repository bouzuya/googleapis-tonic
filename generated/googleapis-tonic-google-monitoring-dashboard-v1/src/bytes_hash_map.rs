pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod monitoring {
        pub mod dashboard {
            pub mod v1 {
                include!("bytes_hash_map/google.monitoring.dashboard.v1.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

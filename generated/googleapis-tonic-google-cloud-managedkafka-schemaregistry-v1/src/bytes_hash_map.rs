pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod managedkafka {
            pub mod schemaregistry {
                pub mod v1 {
                    include!("bytes_hash_map/google.cloud.managedkafka.schemaregistry.v1.rs");
                }
            }
        }
    }
}

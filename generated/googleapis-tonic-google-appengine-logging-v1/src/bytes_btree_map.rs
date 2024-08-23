pub mod google {
    pub mod appengine {
        pub mod logging {
            pub mod v1 {
                include!("bytes_btree_map/google.appengine.logging.v1.rs");
            }
        }
    }
    pub mod logging {
        pub mod r#type {
            pub use googleapis_tonic_google_logging_type::google::logging::r#type::*;
        }
    }
}

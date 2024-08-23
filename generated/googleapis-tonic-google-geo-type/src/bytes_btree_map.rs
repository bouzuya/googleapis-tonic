pub mod google {
    pub mod geo {
        pub mod r#type {
            include!("bytes_btree_map/google.geo.r#type.rs");
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

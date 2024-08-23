pub mod google {
    pub mod actions {
        pub mod r#type {
            include!("vec_u8_hash_map/google.actions.r#type.rs");
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

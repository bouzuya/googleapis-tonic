pub mod google {
    pub mod actions {
        pub mod r#type {
            include!("vec_u8_btree_map/google.actions.r#type.rs");
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}
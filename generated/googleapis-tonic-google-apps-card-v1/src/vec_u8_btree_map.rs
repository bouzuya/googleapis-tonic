pub mod google {
    pub mod apps {
        pub mod card {
            pub mod v1 {
                include!("vec_u8_btree_map/google.apps.card.v1.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

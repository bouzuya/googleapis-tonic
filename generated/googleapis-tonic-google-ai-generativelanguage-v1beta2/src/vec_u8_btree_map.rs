pub mod google {
    pub mod ai {
        pub mod generativelanguage {
            pub mod v1beta2 {
                include!("vec_u8_btree_map/google.ai.generativelanguage.v1beta2.rs");
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
}

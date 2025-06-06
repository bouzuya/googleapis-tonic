pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod sql {
            pub mod v1 {
                include!("vec_u8_btree_map/google.cloud.sql.v1.rs");
            }
        }
    }
}

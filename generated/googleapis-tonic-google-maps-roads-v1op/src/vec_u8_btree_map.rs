pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod maps {
        pub mod roads {
            pub mod v1op {
                include!("vec_u8_btree_map/google.maps.roads.v1op.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

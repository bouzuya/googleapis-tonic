pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod maps {
        pub mod playablelocations {
            pub mod v3 {
                pub mod sample {
                    include!("vec_u8_btree_map/google.maps.playablelocations.v3.sample.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

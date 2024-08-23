pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod geo {
        pub mod r#type {
            pub use googleapis_tonic_google_geo_type::google::geo::r#type::*;
        }
    }
    pub mod maps {
        pub mod addressvalidation {
            pub mod v1 {
                include!("bytes_btree_map/google.maps.addressvalidation.v1.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

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
        pub mod geocode {
            pub mod v4 {
                include!("bytes_btree_map/google.maps.geocode.v4.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

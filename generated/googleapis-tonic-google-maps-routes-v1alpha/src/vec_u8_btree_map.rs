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
        pub mod routes {
            pub mod v1 {
                pub use googleapis_tonic_google_maps_routes_v1::google::maps::routes::v1::*;
            }
            pub mod v1alpha {
                include!("vec_u8_btree_map/google.maps.routes.v1alpha.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

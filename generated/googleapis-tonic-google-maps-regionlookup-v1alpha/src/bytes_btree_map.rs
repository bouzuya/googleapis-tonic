pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod maps {
        pub mod regionlookup {
            pub mod v1alpha {
                include!("bytes_btree_map/google.maps.regionlookup.v1alpha.rs");
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

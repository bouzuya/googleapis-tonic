pub mod google {
    pub mod ads {
        pub mod admob {
            pub mod v1 {
                include!("vec_u8_hash_map/google.ads.admob.v1.rs");
            }
        }
    }
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

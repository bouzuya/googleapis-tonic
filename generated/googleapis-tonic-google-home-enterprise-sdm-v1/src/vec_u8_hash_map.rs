pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod home {
        pub mod enterprise {
            pub mod sdm {
                pub mod v1 {
                    include!("vec_u8_hash_map/google.home.enterprise.sdm.v1.rs");
                }
            }
        }
    }
}

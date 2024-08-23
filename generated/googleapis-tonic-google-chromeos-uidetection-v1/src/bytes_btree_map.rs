pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod chromeos {
        pub mod uidetection {
            pub mod v1 {
                include!("bytes_btree_map/google.chromeos.uidetection.v1.rs");
            }
        }
    }
}

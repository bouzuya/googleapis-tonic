pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod firebase {
        pub mod fcm {
            pub mod connection {
                pub mod v1alpha1 {
                    include!("vec_u8_btree_map/google.firebase.fcm.connection.v1alpha1.rs");
                }
            }
        }
    }
}

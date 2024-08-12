pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod firestore {
        pub mod bundle {
            include!("vec_u8_btree_map/google.firestore.bundle.rs");
        }
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_firestore_v1::google::firestore::v1::*;
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

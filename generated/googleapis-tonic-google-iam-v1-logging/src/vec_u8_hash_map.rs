pub mod google {
    pub mod iam {
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_iam_v1::google::iam::v1::*;
            pub mod logging {
                include!("vec_u8_hash_map/google.iam.v1.logging.rs");
            }
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod secrets {
            pub mod v1beta1 {
                include!("bytes_hash_map/google.cloud.secrets.v1beta1.rs");
            }
        }
    }
    pub mod iam {
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod iam {
        pub mod admin {
            pub mod v1 {
                include!("vec_u8_btree_map/google.iam.admin.v1.rs");
            }
        }
        pub mod v1 {
            pub use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

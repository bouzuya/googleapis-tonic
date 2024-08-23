pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod iam {
        pub mod v1 {
            pub use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod spanner {
        pub mod admin {
            pub mod database {
                pub mod v1 {
                    include!("bytes_btree_map/google.spanner.admin.database.v1.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

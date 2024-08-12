pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod iam {
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod spanner {
        pub mod admin {
            pub mod database {
                pub mod v1 {
                    pub(crate) use googleapis_tonic_google_spanner_admin_database_v1::google::spanner::admin::database::v1::*;
                }
            }
            pub mod instance {
                pub mod v1 {
                    pub(crate) use googleapis_tonic_google_spanner_admin_instance_v1::google::spanner::admin::instance::v1::*;
                }
            }
        }
        pub mod executor {
            pub mod v1 {
                include!("bytes_hash_map/google.spanner.executor.v1.rs");
            }
        }
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_spanner_v1::google::spanner::v1::*;
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

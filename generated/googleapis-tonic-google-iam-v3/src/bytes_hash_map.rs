pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod iam {
        pub mod v3 {
            include!("bytes_hash_map/google.iam.v3.rs");
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub use googleapis_tonic_google_type::google::r#type::*;
    }
}

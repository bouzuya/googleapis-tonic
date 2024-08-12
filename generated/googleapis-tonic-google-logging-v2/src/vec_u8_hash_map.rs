pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod logging {
        pub mod r#type {
            pub(crate) use googleapis_tonic_google_logging_type::google::logging::r#type::*;
        }
        pub mod v2 {
            include!("vec_u8_hash_map/google.logging.v2.rs");
        }
    }
    pub mod longrunning {
        pub(crate) use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

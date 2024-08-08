pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
        pub mod servicecontrol {
            pub mod v1 {
                include!("bytes_hash_map/google.api.servicecontrol.v1.rs");
            }
        }
    }
    pub mod logging {
        pub mod r#type {
            pub(crate) use googleapis_tonic_google_logging_type::google::logging::r#type::*;
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

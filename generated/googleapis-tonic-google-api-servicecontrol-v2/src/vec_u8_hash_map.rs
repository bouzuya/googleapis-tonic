pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
        pub mod servicecontrol {
            pub mod v2 {
                include!("vec_u8_hash_map/google.api.servicecontrol.v2.rs");
            }
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
        pub mod context {
            pub(crate) use googleapis_tonic_google_rpc_context::google::rpc::context::*;
        }
    }
}

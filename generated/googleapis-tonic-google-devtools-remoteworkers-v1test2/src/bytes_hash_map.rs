pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod devtools {
        pub mod remoteworkers {
            pub mod v1test2 {
                include!("bytes_hash_map/google.devtools.remoteworkers.v1test2.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

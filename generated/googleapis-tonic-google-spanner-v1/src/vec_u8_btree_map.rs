pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod spanner {
        pub mod v1 {
            include!("vec_u8_btree_map/google.spanner.v1.rs");
        }
    }
}

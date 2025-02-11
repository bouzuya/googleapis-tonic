pub mod google {
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod storage {
        pub mod platformlogs {
            pub mod v1 {
                include!("vec_u8_btree_map/google.storage.platformlogs.v1.rs");
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub mod expr {
            pub mod v1beta1 {
                include!("vec_u8_hash_map/google.api.expr.v1beta1.rs");
            }
        }
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

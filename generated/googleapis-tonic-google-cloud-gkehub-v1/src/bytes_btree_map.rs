pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod gkehub {
            pub mod configmanagement {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.gkehub.configmanagement.v1.rs");
                }
            }
            pub mod multiclusteringress {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.gkehub.multiclusteringress.v1.rs");
                }
            }
            pub mod rbacrolebindingactuation {
                pub mod v1 {
                    include!("bytes_btree_map/google.cloud.gkehub.rbacrolebindingactuation.v1.rs");
                }
            }
            pub mod v1 {
                include!("bytes_btree_map/google.cloud.gkehub.v1.rs");
            }
        }
    }
    pub mod longrunning {
        pub use googleapis_tonic_google_longrunning::google::longrunning::*;
    }
    pub mod rpc {
        pub use googleapis_tonic_google_rpc::google::rpc::*;
    }
}

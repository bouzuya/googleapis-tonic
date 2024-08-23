pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod gkehub {
            pub mod cloudauditlogging {
                pub mod v1alpha {
                    include!("vec_u8_btree_map/google.cloud.gkehub.cloudauditlogging.v1alpha.rs");
                }
            }
            pub mod configmanagement {
                pub mod v1alpha {
                    include!("vec_u8_btree_map/google.cloud.gkehub.configmanagement.v1alpha.rs");
                }
            }
            pub mod metering {
                pub mod v1alpha {
                    include!("vec_u8_btree_map/google.cloud.gkehub.metering.v1alpha.rs");
                }
            }
            pub mod multiclusteringress {
                pub mod v1alpha {
                    include!("vec_u8_btree_map/google.cloud.gkehub.multiclusteringress.v1alpha.rs");
                }
            }
            pub mod servicemesh {
                pub mod v1alpha {
                    include!("vec_u8_btree_map/google.cloud.gkehub.servicemesh.v1alpha.rs");
                }
            }
            pub mod v1alpha {
                include!("vec_u8_btree_map/google.cloud.gkehub.v1alpha.rs");
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

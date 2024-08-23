pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod cloud {
        pub mod gkehub {
            pub mod configmanagement {
                pub mod v1beta {
                    include!("bytes_hash_map/google.cloud.gkehub.configmanagement.v1beta.rs");
                }
            }
            pub mod metering {
                pub mod v1beta {
                    include!("bytes_hash_map/google.cloud.gkehub.metering.v1beta.rs");
                }
            }
            pub mod multiclusteringress {
                pub mod v1beta {
                    include!("bytes_hash_map/google.cloud.gkehub.multiclusteringress.v1beta.rs");
                }
            }
            pub mod servicemesh {
                pub mod v1beta {
                    pub use googleapis_tonic_google_cloud_gkehub_servicemesh_v1beta::google::cloud::gkehub::servicemesh::v1beta::*;
                }
            }
            pub mod v1beta {
                include!("bytes_hash_map/google.cloud.gkehub.v1beta.rs");
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

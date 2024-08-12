pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod devtools {
        pub mod containeranalysis {
            pub mod v1 {
                include!("vec_u8_btree_map/google.devtools.containeranalysis.v1.rs");
            }
        }
    }
    pub mod iam {
        pub mod v1 {
            pub(crate) use googleapis_tonic_google_iam_v1::google::iam::v1::*;
        }
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}
pub mod grafeas {
    pub mod v1 {
        pub(crate) use googleapis_tonic_grafeas_v1::grafeas::v1::*;
    }
}

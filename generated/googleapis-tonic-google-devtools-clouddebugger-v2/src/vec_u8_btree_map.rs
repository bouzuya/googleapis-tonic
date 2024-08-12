pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod devtools {
        pub mod clouddebugger {
            pub mod v2 {
                include!("vec_u8_btree_map/google.devtools.clouddebugger.v2.rs");
            }
        }
        pub mod source {
            pub mod v1 {
                pub(crate) use googleapis_tonic_google_devtools_source_v1::google::devtools::source::v1::*;
            }
        }
    }
}

pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod card {
            pub mod v1 {
                pub(crate) use googleapis_tonic_google_apps_card_v1::google::apps::card::v1::*;
            }
        }
    }
    pub mod chat {
        pub mod v1 {
            include!("bytes_hash_map/google.chat.v1.rs");
        }
    }
    pub mod rpc {
        pub(crate) use googleapis_tonic_google_rpc::google::rpc::*;
    }
    pub mod r#type {
        pub(crate) use googleapis_tonic_google_type::google::r#type::*;
    }
}

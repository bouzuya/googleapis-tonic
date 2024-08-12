pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod script {
            pub mod r#type {
                pub(crate) use googleapis_tonic_google_apps_script_type::google::apps::script::r#type::*;
                pub mod docs {
                    include!("vec_u8_hash_map/google.apps.script.r#type.docs.rs");
                }
            }
        }
    }
}

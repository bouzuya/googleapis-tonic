pub mod google {
    pub mod api {
        pub use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod script {
            pub mod r#type {
                pub use googleapis_tonic_google_apps_script_type::google::apps::script::r#type::*;
                pub mod calendar {
                    include!("vec_u8_btree_map/google.apps.script.r#type.calendar.rs");
                }
            }
        }
    }
}

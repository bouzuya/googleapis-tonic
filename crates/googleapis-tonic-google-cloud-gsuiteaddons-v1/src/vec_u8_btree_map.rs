pub mod google {
    pub mod api {
        pub(crate) use googleapis_tonic_google_api::google::api::*;
    }
    pub mod apps {
        pub mod script {
            pub mod r#type {
                pub(crate) use googleapis_tonic_google_apps_script_type::google::apps::script::r#type::*;
                pub mod calendar {
                    pub(crate) use googleapis_tonic_google_apps_script_type_calendar::google::apps::script::r#type::calendar::*;
                }
                pub mod docs {
                    pub(crate) use googleapis_tonic_google_apps_script_type_docs::google::apps::script::r#type::docs::*;
                }
                pub mod drive {
                    pub(crate) use googleapis_tonic_google_apps_script_type_drive::google::apps::script::r#type::drive::*;
                }
                pub mod gmail {
                    pub(crate) use googleapis_tonic_google_apps_script_type_gmail::google::apps::script::r#type::gmail::*;
                }
                pub mod sheets {
                    pub(crate) use googleapis_tonic_google_apps_script_type_sheets::google::apps::script::r#type::sheets::*;
                }
                pub mod slides {
                    pub(crate) use googleapis_tonic_google_apps_script_type_slides::google::apps::script::r#type::slides::*;
                }
            }
        }
    }
    pub mod cloud {
        pub mod gsuiteaddons {
            pub mod v1 {
                include!("vec_u8_btree_map/google.cloud.gsuiteaddons.v1.rs");
            }
        }
    }
}

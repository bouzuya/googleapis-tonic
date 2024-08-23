pub mod google {
    pub mod apps {
        pub mod script {
            pub mod r#type {
                pub use googleapis_tonic_google_apps_script_type::google::apps::script::r#type::*;
                pub mod drive {
                    include!("bytes_btree_map/google.apps.script.r#type.drive.rs");
                }
            }
        }
    }
}

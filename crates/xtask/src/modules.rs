use std::str::FromStr as _;

use crate::{feature_name::FeatureName, ident::Ident, module::Module};

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Modules(Vec<Module>);

impl<'a> IntoIterator for &'a Modules {
    type Item = &'a Module;
    type IntoIter = std::slice::Iter<'a, Module>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Modules {
    pub fn from_file_names(paths: &[String]) -> Self {
        let mut modules = Modules::default();
        for path in paths {
            let idents = path
                .split('.')
                .collect::<Vec<&str>>()
                .into_iter()
                .filter(|s| s != &"rs")
                .map(Ident::from_str)
                .collect::<anyhow::Result<Vec<Ident>>>()
                .expect("path to be valid ident");
            if idents.is_empty() {
                continue;
            }

            let feature_name = FeatureName::from(idents.clone());

            let mut module = modules.get_or_insert(&idents[0]);
            module.add_feature(feature_name.clone());
            for ident in idents.into_iter().skip(1) {
                module = module.modules_mut().get_or_insert(&ident);
                module.add_feature(feature_name.clone());
            }
            module.add_include();
            module.add_feature(feature_name.clone());
        }
        modules
    }

    pub fn get_or_insert(&mut self, ident: &Ident) -> &mut Module {
        match self.0.iter().position(|m| m.ident() == ident) {
            Some(index) => self.0.get_mut(index).expect("mod is included"),
            None => {
                self.0.push(Module::new(ident.to_owned()));
                self.0.last_mut().expect("mods is not empty")
            }
        }
    }

    pub fn to_rs_file_content(&self) -> String {
        fn dfs(modules: &Modules, c: &mut Vec<String>, s: &mut String) {
            let indent = "    ";
            for module in modules {
                let features = module.features();
                if !features.is_empty() {
                    s.push_str(&format!("{}#[cfg(\n", indent.repeat(c.len())));
                    s.push_str(&format!("{}any(\n", indent.repeat(c.len() + 1)));
                    for feature in features {
                        s.push_str(&format!(
                            "{}feature=\"{}\",\n",
                            indent.repeat(c.len() + 2),
                            feature
                        ));
                    }
                    s.push_str(&format!("{})\n", indent.repeat(c.len() + 1)));
                    s.push_str(&format!("{})]\n", indent.repeat(c.len())));
                }
                s.push_str(&format!(
                    "{}pub mod {} {{\n",
                    indent.repeat(c.len()),
                    module.ident().as_str()
                ));
                c.push(module.ident().to_string());
                if module.include() {
                    s.push_str(&format!(
                        "{}include!(\"{}.rs\");\n",
                        indent.repeat(c.len()),
                        c.join("."),
                    ));
                }
                dfs(module.modules(), c, s);
                c.pop();
                s.push_str(&format!("{}}}\n", indent.repeat(c.len())));
            }
        }

        let mut s = String::new();
        let mut c = vec![];
        dfs(self, &mut c, &mut s);
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::Modules;

    #[test]
    fn test_modules() {
        let paths = [
            "google.firestore.rs",
            "google.firestore.v1.rs",
            "google.firestore.v1beta1.rs",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
        assert_eq!(
            Modules::from_file_names(&paths).to_rs_file_content(),
            r#"#[cfg(
    any(
        feature="google-firestore",
        feature="google-firestore-v1",
        feature="google-firestore-v1beta1",
    )
)]
pub mod google {
    #[cfg(
        any(
            feature="google-firestore",
            feature="google-firestore-v1",
            feature="google-firestore-v1beta1",
        )
    )]
    pub mod firestore {
        include!("google.firestore.rs");
        #[cfg(
            any(
                feature="google-firestore-v1",
            )
        )]
        pub mod v1 {
            include!("google.firestore.v1.rs");
        }
        #[cfg(
            any(
                feature="google-firestore-v1beta1",
            )
        )]
        pub mod v1beta1 {
            include!("google.firestore.v1beta1.rs");
        }
    }
}
"#
        );
    }
}

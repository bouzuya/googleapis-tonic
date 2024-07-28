use crate::module::Module;

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
            let names = path.split('.').collect::<Vec<&str>>();
            if names.is_empty() {
                continue;
            }
            let mut module = modules.get_or_insert(names[0]);
            for name in names.into_iter().skip(1) {
                if name == "rs" {
                    module.add_include();
                    break;
                } else {
                    module = module.modules_mut().get_or_insert(name);
                }
            }
        }
        modules
    }

    pub fn get_or_insert(&mut self, ident: &str) -> &mut Module {
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
                s.push_str(&format!(
                    "{}pub mod {} {{\n",
                    indent.repeat(c.len()),
                    module.ident()
                ));
                c.push(module.ident().to_owned());
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
            r#"pub mod google {
    pub mod firestore {
        include!("google.firestore.rs");
        pub mod v1 {
            include!("google.firestore.v1.rs");
        }
        pub mod v1beta1 {
            include!("google.firestore.v1beta1.rs");
        }
    }
}
"#
        );
    }
}

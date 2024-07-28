use std::collections::BTreeSet;

use crate::modules::Modules;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Module {
    ident: String,
    features: BTreeSet<String>,
    include: bool,
    modules: Modules,
}

impl Module {
    pub fn new(ident: String) -> Self {
        Self {
            ident,
            features: BTreeSet::new(),
            include: false,
            modules: Modules::default(),
        }
    }

    pub fn with_include(ident: String) -> Self {
        Self {
            ident,
            features: BTreeSet::new(),
            include: true,
            modules: Modules::default(),
        }
    }

    pub fn add_include(&mut self) {
        self.include = true;
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn include(&self) -> bool {
        self.include
    }

    pub fn modules(&self) -> &Modules {
        &self.modules
    }

    pub fn modules_mut(&mut self) -> &mut Modules {
        &mut self.modules
    }
}

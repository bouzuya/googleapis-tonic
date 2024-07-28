use std::collections::BTreeSet;

use crate::{feature_name::FeatureName, ident::Ident, modules::Modules};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Module {
    ident: Ident,
    features: BTreeSet<FeatureName>,
    include: bool,
    modules: Modules,
}

impl Module {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            features: BTreeSet::new(),
            include: false,
            modules: Modules::default(),
        }
    }

    pub fn with_include(ident: Ident) -> Self {
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

    pub fn ident(&self) -> &Ident {
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

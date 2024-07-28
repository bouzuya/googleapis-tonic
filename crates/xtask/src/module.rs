use crate::modules::Modules;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Module {
    ident: String,
    include: bool,
    modules: Modules,
}

impl Module {
    pub fn new(ident: String) -> Self {
        Self {
            ident,
            include: false,
            modules: Modules::default(),
        }
    }

    pub fn with_include(ident: String) -> Self {
        Self {
            ident,
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

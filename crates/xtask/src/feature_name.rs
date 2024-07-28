use std::fmt::Display;

use crate::ident::Ident;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct FeatureName(Vec<Ident>);

impl Display for FeatureName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(Ident::to_string)
            .map(|s| s.replace("r#", ""))
            .collect::<Vec<String>>()
            .join("-")
            .fmt(f)
    }
}

impl From<Vec<Ident>> for FeatureName {
    fn from(idents: Vec<Ident>) -> Self {
        Self(idents)
    }
}

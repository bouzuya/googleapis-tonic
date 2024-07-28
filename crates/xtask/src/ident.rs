use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ident(String);

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Ident {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl From<Ident> for String {
    fn from(ident: Ident) -> Self {
        ident.0
    }
}

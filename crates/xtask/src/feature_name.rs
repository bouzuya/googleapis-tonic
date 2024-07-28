use crate::ident::Ident;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct FeatureName(Vec<Ident>);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GoogleapisVersion(String);

impl std::fmt::Display for GoogleapisVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for GoogleapisVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TOOD: check format
        Ok(Self(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::GoogleapisVersion;

    use std::str::FromStr as _;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "906736032699b7e943ef2155edbda05470723647";
        let v = GoogleapisVersion::from_str(s)?;
        assert_eq!(v.to_string(), s);
        Ok(())
    }
}

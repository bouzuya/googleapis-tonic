use semver::BuildMetadata;
use semver::Prerelease;
use semver::Version;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateVersion(Version);

impl std::fmt::Display for CrateVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for CrateVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version = Version::from_str(s)?;
        anyhow::ensure!(version.pre == Prerelease::EMPTY);
        anyhow::ensure!(version.build == BuildMetadata::EMPTY);
        Ok(Self(version))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test_conversion_stirng() -> anyhow::Result<()> {
        assert_eq!(CrateVersion::from_str("1.2.3")?.to_string(), "1.2.3");
        assert!(CrateVersion::from_str("1.2.3-pre").is_err());
        assert!(CrateVersion::from_str("1.2.3+build").is_err());
        assert!(CrateVersion::from_str("1.2.3-pre+build").is_err());
        Ok(())
    }
}

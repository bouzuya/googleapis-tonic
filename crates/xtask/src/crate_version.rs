use semver::BuildMetadata;
use semver::Prerelease;
use semver::Version;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateVersion(Version);

impl CrateVersion {
    pub fn increment_minor(&self) -> Self {
        let v = &self.0;
        let next_version = Version::new(v.major, v.minor + 1, v.patch);
        Self(next_version)
    }
}

impl std::default::Default for CrateVersion {
    fn default() -> Self {
        Self(Version::new(0, 0, 0))
    }
}

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

    #[test]
    fn test_default() {
        assert_eq!(CrateVersion::default().to_string(), "0.0.0");
    }

    #[test]
    fn test_increment_minor() -> anyhow::Result<()> {
        let v = CrateVersion::from_str("1.2.3")?;
        assert_eq!(v.increment_minor().to_string(), "1.3.3");
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GoogleapisVersion(String);

impl GoogleapisVersion {
    // TODO: remove I/O
    pub fn load_from_googleapis_dir<P>(googleapis_dir: P) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        use std::process::Command;
        use std::str::FromStr as _;

        let googleapis_dir = googleapis_dir.as_ref();
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(googleapis_dir)
            .output()?;
        anyhow::ensure!(output.status.success());
        let s = String::from_utf8(output.stdout)?;
        let version = GoogleapisVersion::from_str(s.trim_end())?;
        Ok(version)
    }
}

impl std::fmt::Display for GoogleapisVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for GoogleapisVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::ensure!(s.len() == 40);
        // allow ascii lowercase hex digit
        anyhow::ensure!(s
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, 'a'..='f')));
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

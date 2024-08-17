use sha1::Digest;
use sha1::Sha1;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sha1Hash(String);

impl Sha1Hash {
    pub fn digest(s: &str) -> Self {
        Self(hex::encode(Sha1::digest(s)))
    }
}

impl std::fmt::Display for Sha1Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Sha1Hash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::ensure!(s.len() == 40);
        // allow lowercase hex
        anyhow::ensure!(s
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, 'a'..='f')));
        Ok(Self(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test_digest() -> anyhow::Result<()> {
        assert_eq!(
            Sha1Hash::digest("package foo;"),
            Sha1Hash::from_str("35bcb46da375fe8a3772acc187ec66acea8e5cfa")?,
        );
        Ok(())
    }

    #[test]
    fn test_string_conversion() -> anyhow::Result<()> {
        let s = "35bcb46da375fe8a3772acc187ec66acea8e5cfa";
        assert_eq!(Sha1Hash::from_str(s)?.to_string(), s);
        Ok(())
    }
}

use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageName(String);

impl std::fmt::Display for PackageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PackageName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_full_ident(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(anyhow::anyhow!("invalid fullIdent"))
        }
    }
}

// <https://protobuf.dev/reference/protobuf/proto3-spec/#identifiers>
// ident = letter { letter | decimalDigit | "_" }
// fullIdent = ident { "." ident }
fn is_full_ident(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
        && {
            let idents = s.split('.').collect::<Vec<&str>>();
            idents.into_iter().all(is_ident)
        }
}

// <https://protobuf.dev/reference/protobuf/proto3-spec/#identifiers>
// ident = letter { letter | decimalDigit | "_" }
//
// <https://protobuf.dev/reference/protobuf/proto3-spec/#letters_and_digits>
// decimalDigit = "0" ... "9"
// letter = "A" ... "Z" | "a" ... "z"
fn is_ident(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    chars.iter().all(|c| c.is_ascii_alphanumeric() || *c == '_')
        && match chars.first() {
            None => {
                // empty ident is invalid
                false
            }
            Some(&c) => {
                // first char is letter
                c.is_ascii_alphabetic()
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ident() {
        assert!(!is_ident(""));
        assert!(!is_ident("1"));
        assert!(!is_ident("_"));
        assert!(is_ident("a"));
        assert!(is_ident("a1"));
        assert!(is_ident("a_"));
        assert!(is_ident("ab"));
    }

    #[test]
    fn test_is_full_ident() {
        assert!(!is_full_ident(""));
        assert!(!is_full_ident("."));
        assert!(is_full_ident("a"));
        assert!(!is_full_ident("a."));
        assert!(is_full_ident("a.b"));
    }
}

use std::{fmt, str};

#[derive(Debug)]
pub struct SemanticVersion {
    major: u64,
    minor: u64,
    patch: Option<u64>,
    pre_release: Option<PreRelease>,
    build_metadata: Option<BuildMetadata>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreRelease {
    raw: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BuildMetadata {
    raw: String,
}

impl str::FromStr for SemanticVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.trim_start_matches('v');
        let v: Vec<&str> = v.split('.').collect();
        if v.len() < 2 {
            return Err(String::from("invalid format. <major>.<minor>.<path>"));
        }
        // TODO: handle error
        let (major, minor, patch) = (
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v.get(2).map(|patch| patch.parse().unwrap()),
        );

        // TODO: handle pre_release & metadata

        Ok(Self {
            major,
            minor,
            patch,
            ..Default::default()
        })
    }
}

impl Default for SemanticVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: Some(0),
            pre_release: None,
            build_metadata: None,
        }
    }
}

impl PartialEq for SemanticVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.pre_release == other.pre_release
            && self.build_metadata == other.build_metadata
    }
}

impl fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: handle pre_release & metadata
        write!(f, "v{}.{}", self.major, self.minor)?;
        if let Some(patch) = self.patch {
            write!(f, ".{}", patch)?;
        }
        Ok(())
    }
}

impl SemanticVersion {
    pub fn format_without_prefix(&self) -> String {
        format!("{}", self).trim_start_matches('v').to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            "v0.1.2".parse::<SemanticVersion>().ok(),
            Some(SemanticVersion {
                major: 0,
                minor: 1,
                patch: Some(2),
                ..Default::default()
            })
        );
        // without prefix v
        assert_eq!(
            "0.1.2".parse::<SemanticVersion>().ok(),
            Some(SemanticVersion {
                major: 0,
                minor: 1,
                patch: Some(2),
                ..Default::default()
            })
        );
        assert_eq!(
            "0.1".parse::<SemanticVersion>().ok(),
            Some(SemanticVersion {
                major: 0,
                minor: 1,
                patch: None,
                ..Default::default()
            })
        );
    }

    #[test]
    fn format() {
        let v = SemanticVersion {
            major: 1,
            minor: 2,
            patch: Some(3),
            ..Default::default()
        };

        assert_eq!(v.format_without_prefix(), "1.2.3", );

        let v = SemanticVersion {
            major: 1,
            minor: 2,
            patch: None,
            ..Default::default()
        };

        assert_eq!(v.format_without_prefix(), "1.2", );
    }
}

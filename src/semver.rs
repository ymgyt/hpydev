#[derive(Debug)]
pub struct SemanticVersion {
    major: u64,
    minor: u64,
    patch: u64,
    pre_release: Option<PreRelease>,
    build_metadata: Option<BuildMetadata>,
}

#[derive(Debug,PartialEq,Eq)]
pub struct PreRelease {
    raw: String,
}

#[derive(Debug,PartialEq,Eq)]
pub struct BuildMetadata {
    raw: String,
}

impl std::str::FromStr for SemanticVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.trim_start_matches('v');
        let v: Vec<&str> = v.split(".").collect();
        if v.len() < 3 {
            return Err(String::from("invalid format. <major>.<minor>.<path>"))
        }
        // TODO: handle error
        let (major,minor,patch) = (
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v[2].parse().unwrap(),
        );

        // TODO: handle pre_release & metadata

        Ok(Self{
            major,minor,patch,
            ..Default::default()
        })

    }
}

impl Default for SemanticVersion {
    fn default() -> Self {
        Self{
            major: 0,
            minor: 1,
            patch: 0,
            pre_release: None,
            build_metadata: None,
        }
    }
}

impl PartialEq for SemanticVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major &&
            self.minor == other.minor &&
            self.patch == other.patch &&
            self.pre_release == other.pre_release &&
            self.build_metadata == other.build_metadata
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            "v0.1.2".parse::<SemanticVersion>().ok(),
            Some(SemanticVersion{
                major: 0,
                minor: 1,
                patch: 2,
                ..Default::default()
            })
        );
        // without prefix v
        assert_eq!(
            "0.1.2".parse::<SemanticVersion>().ok(),
            Some(SemanticVersion{
                major: 0,
                minor: 1,
                patch: 2,
                ..Default::default()
            })
        );
    }
}

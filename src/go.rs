use crate::{cli, config, prelude::*, semver::SemanticVersion, util, Os};
use std::{convert::TryFrom, path, str::FromStr};
use tracing::{debug, info};

#[derive(Debug)]
pub enum Arch {
    Amd64,
}

impl FromStr for Arch {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "amd64" => Ok(Arch::Amd64),
            etc => Err(anyhow::anyhow!("undefined arch {}", etc)),
        }
    }
}

pub struct Operator {}

#[derive(Debug)]
pub struct InstallParam {
    version: SemanticVersion,
    arch: Arch,
    dest: path::PathBuf,
}

impl TryFrom<cli::go::Install> for InstallParam {
    type Error = anyhow::Error;
    fn try_from(v: cli::go::Install) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            version: v
                .version
                .parse()
                .map_err(|e| anyhow::anyhow!("semantic version. {}", e))?,
            arch: v.arch.parse()?,
            dest: v.dest,
        })
    }
}

impl Operator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn install(self, param: InstallParam) -> Result<()> {
        debug!("{:#?}", param);

        let os = util::get_os().unwrap();
        let mut endpoint = config::go::ARCHIVE_URL.clone();
        endpoint
            .path_segments_mut()
            .unwrap()
            .push(make_archive_segment(&param.version, &os, &param.arch).as_str());

        // current決め打ち。tmp directory作成してそっちに作ったほうが行儀いいかも。
        let archive_dest = "./go_archive.tar.gz";

        let response = reqwest::get(endpoint.clone()).await?;
        if let Err(e) = response.error_for_status_ref() {
            return Err(e.into())
        }

        let mut stream = match  reqwest::get(endpoint.clone()).await?.error_for_status() {
           Ok(res) => res.bytes_stream(),
            Err(err) => return Err(err.into()),
        };

        use tokio::io::AsyncWriteExt;
        use tokio::stream::StreamExt;
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(archive_dest)
            .await?;

        info!("successfully download {} to {}", endpoint, archive_dest);

        while let Some(Ok(v)) = stream.next().await {
            file.write_all(&v).await?;
        }

        util::extract_tar_gz(archive_dest, param.dest.as_path()).await?;

        // TODO: remove archive file

        Ok(())
    }
}

//
// ex. https://golang.org/dl/go1.15.1.darwin-amd64.tar.gz の最後のsegmentを作る
fn make_archive_segment(ver: &SemanticVersion, os: &Os, arch: &Arch) -> String {
    let os = match os {
        Os::Darwin => "darwin",
        Os::Linux => "linux",
    };
    let arch = match arch {
        Arch::Amd64 => "amd64",
    };

    format!(
        "go{version}.{os}-{arch}.tar.gz",
        version = ver.format_without_prefix(),
        os = os,
        arch = arch,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_archive_segment() {
        // (arg, expect)
        vec![(
            (
                SemanticVersion::from_str("v1.15.0").unwrap(),
                &Os::Darwin,
                &Arch::Amd64,
            ),
            "go1.15.0.darwin-amd64.tar.gz",
        )]
        .into_iter()
        .for_each(|t| {
            let arg = t.0;
            let expect = String::from(t.1);
            assert_eq!(make_archive_segment(&arg.0, arg.1, arg.2), expect);
        });
    }
}

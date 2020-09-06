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

        info!("{:#?}", endpoint);

        let mut stream = reqwest::get(endpoint).await?.bytes_stream();

        use tokio::io::AsyncWriteExt;
        use tokio::stream::StreamExt;
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(param.dest)
            .await?;

        while let Some(Ok(v)) = stream.next().await {
            file.write_all(&v).await?;
        }

        Ok(())
    }
}

//
// ex. https://golang.org/dl/go1.15.1.darwin-amd64.tar.gz
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

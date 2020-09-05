use crate::{prelude::*, cli,config,semver::SemanticVersion};
use tracing::{debug};
use std::{path,convert::TryFrom};

pub struct Operator {}

#[derive(Debug)]
pub struct InstallParam {
    // version型定義したい
    version: SemanticVersion,
    arch: String,
    dest: path::PathBuf,
}

impl TryFrom<cli::go::Install> for InstallParam {
    type Error = anyhow::Error;
    fn try_from(v: cli::go::Install) -> std::result::Result<Self,Self::Error> {
       Ok(Self{
           version: v.version.parse().map_err(|e| anyhow::anyhow!("semantic version. {}", e))?,
           arch: v.arch,
           dest: v.dest,
       })
    }
}

impl Operator {
    pub fn new() -> Self {
        Self{}
    }

    pub async fn install(self, param: InstallParam) -> Result<()> {
       debug!("{:#?}", param);

        // // https://golang.org/dl/go1.15.1.darwin-amd64.tar.gz
        // let mut archive_endpoint = config::go::ARCHIVE_URL.clone();
        // archive_endpoint.path_segments_mut().unwrap().extend(&[
        //
        // ])
        //
        Ok(())
    }
}


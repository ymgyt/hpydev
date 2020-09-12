use crate::{cli, config, fsops, prelude::*, semver::SemanticVersion, util, Os};
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
    fn try_from(v: cli::go::Install) -> core::result::Result<Self, Self::Error> {
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

#[derive(Debug)]
pub struct UninstallParam {
    go_root: path::PathBuf,
    skip_remove_prompt: bool,
}

impl TryFrom<cli::go::Uninstall> for UninstallParam {
    type Error = anyhow::Error;
    fn try_from(v: cli::go::Uninstall) -> core::result::Result<Self, Self::Error> {
        Ok(Self {
            go_root: v.go_root,
            skip_remove_prompt: v.skip_prompt,
        })
    }
}

impl Default for Operator {
    fn default() -> Self {
        Self {}
    }
}

impl Operator {
    pub fn new() -> Self {
        Operator::default()
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

        info!("downloading {} ...", &endpoint);
        let mut stream = match reqwest::get(endpoint.clone()).await?.error_for_status() {
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

        while let Some(Ok(v)) = stream.next().await {
            file.write_all(&v).await?;
        }

        info!("successfully download {} to {}", endpoint, archive_dest);

        fsops::extract_tar_gz(archive_dest, param.dest.as_path()).await?;
        fsops::remove(archive_dest).await?;

        // TODO: go processを表現して、Go::new().version()とかしたい
        info!("go successfully installed. try $(which go) version");

        Ok(())
    }

    pub async fn uninstall(self, param: UninstallParam) -> Result<()> {
        debug!("{:#?}", param);

        if !param.skip_remove_prompt {
            let ok = dialoguer::Confirm::new()
                .with_prompt(format!(
                    "delete {:?} recursively",
                    param.go_root.as_path().as_os_str()
                ))
                .interact()?;
            if !ok {
                info!("canceled");
                return Ok(());
            }
        }

        // TODO: ここでgo clean -cache等のcleanup系のタスクを実行してもいいかも
        // TODO: go_rootにgoがinstallされているか確認したい

        fsops::remove_dir_recursively(param.go_root.as_path()).await?;
        info!("successfully delete {:?}", param.go_root);

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

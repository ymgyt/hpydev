use crate::config;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "golang action")]
pub struct Go {
    #[structopt(subcommand)]
    pub action: Action,
}

#[derive(StructOpt, Debug, Clone)]
pub enum Action {
    Install(Install),
    Uninstall(Uninstall),
    Upgrade(Upgrade),
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "install golang")]
pub struct Install {
    #[structopt(long = "version", help = "version to install.")]
    pub version: String,

    #[structopt(
        long = "arch",
        help = "GO ARCH",
        default_value(config::go::DEFAULT_ARCH)
    )]
    pub arch: String,

    #[structopt(
        long = "dest",
        help = "directory to which install go.",
        default_value(config::go::DEFAULT_DEST)
    )]
    pub dest: PathBuf,
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "uninstall golang")]
pub struct Uninstall {}

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "upgrade golang version")]
pub struct Upgrade {}

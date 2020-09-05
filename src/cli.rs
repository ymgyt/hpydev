pub mod go;

use structopt::{clap, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "hpydev",
    about = "utility cli for happy developing",
    version(env!("CARGO_PKG_VERSION")),
    setting(clap::AppSettings::ArgRequiredElseHelp),
    global_settings(&[
        clap::AppSettings::ColoredHelp,
        clap::AppSettings::ColorAuto,
        clap::AppSettings::VersionlessSubcommands,
        clap::AppSettings::DisableHelpSubcommand,
        clap::AppSettings::DeriveDisplayOrder,
    ]),
)]
pub struct Opt {
    #[structopt(
        short = "v",
        long = "verbose",
        global = true,
        help = "logging verbose. (v=DEBUG, vv=TRACE)",
        parse(from_occurrences)
    )]
    pub verbose: u8,

    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

#[derive(StructOpt, Debug,Clone)]
pub enum SubCommand {
    Go(crate::cli::go::Go),
}

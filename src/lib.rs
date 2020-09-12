pub mod cli;
pub mod fsops;
pub mod go;

mod config;
mod prelude;
mod semver;
mod util;

pub enum Os {
    Darwin,
    Linux,
}

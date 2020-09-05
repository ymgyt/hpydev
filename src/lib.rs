pub mod cli;
pub mod go;

mod config;
mod util;
mod semver;
mod prelude;

pub enum Os {
    Darwin,
    Linux,
}


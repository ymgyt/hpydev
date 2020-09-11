use lazy_static::lazy_static;
use url::Url;

pub mod go {
    use super::*;

    pub const DEFAULT_ARCH: &'static str = "amd64";
    pub const DEFAULT_DEST: &'static str = "/usr/local";

    lazy_static! {
        pub static ref ARCHIVE_URL: Url = Url::parse("https://golang.org/dl").unwrap();
        // https://golang.org/dl/go1.15.2.darwin-amd64.tar.gz
        // https://golang.org/dl/go1.15.1.darwin-amd64.tar.gz
    }
}

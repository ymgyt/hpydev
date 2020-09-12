//! Provide filesystem operations.

use std::{io, path::Path};

/// Extract src tar.gz file to dest.
/// Equivalent to tar -C <dest> -xzf <src>
pub async fn extract_tar_gz<P1, P2>(src: P1, dest: P2) -> Result<(), anyhow::Error>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    // destが存在していないと
    // tar: could not chdir to <dest> errorになる
    create_dir(dest.as_ref()).await?;

    use tokio::process::Command;
    let status = Command::new("tar")
        .arg("-C")
        .arg(dest.as_ref().as_os_str())
        .arg("-xzf")
        .arg(src.as_ref().as_os_str())
        .spawn()?
        .await;

    tracing::debug!("{:#?}", status);

    Ok(())
}

pub async fn create_dir(path: impl AsRef<Path>) -> io::Result<()> {
    tokio::fs::create_dir_all(path).await
}

pub async fn remove(path: impl AsRef<Path>) -> io::Result<()> {
    tokio::fs::remove_file(path).await
}

/// Equivalent to rm -rf <path>
/// Use carefully!
pub async fn remove_dir_recursively(path: impl AsRef<Path>) -> io::Result<()> {
    tokio::fs::remove_dir_all(path).await
}

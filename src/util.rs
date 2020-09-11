use std::path::Path;

#[cfg(target_os = "macos")]
pub fn get_os() -> Option<crate::Os> {
    Some(crate::Os::Darwin)
}

#[cfg(not(target_os = "macos"))]
pub fn get_os() -> Option<crate::Os> {
    None
}

pub async fn extract_tar_gz<P1, P2>(src: P1, dest: P2) -> Result<(), anyhow::Error>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
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

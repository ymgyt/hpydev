#[cfg(target_os = "macos")]
pub fn get_os() -> Option<crate::Os> {
    Some(crate::Os::Darwin)
}

#[cfg(not(target_os = "macos"))]
pub fn get_os() -> Option<crate::Os> {
    None
}

use std::path::PathBuf;

pub(crate) fn get_home_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home))
}
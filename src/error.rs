use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("Shell detection failed: {0}")]
    DetectionFailed(String),

    #[error("Shell environment variable not found")]
    NoShellVar,

    #[error("Home directory not found")]
    NoHomeDir,

    #[error("Failed to access RC file: {0}")]
    RcFileError(#[from] std::io::Error),

    #[error("Unsupported platform")]
    UnsupportedPlatform,
}
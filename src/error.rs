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

    #[error("Failed to execute shell command")]
    CommandFailed,

    #[error("Invalid UTF-8 in shell output")]
    InvalidUtf8Output,

    #[error("ZDOTDIR environment variable is empty")]
    EmptyZdotdir,

    #[error("Home environment variable is empty")]
    EmptyHomeEnvVar,

    #[error("Home environment and ZDOTDIR variables are empty")]
    EmptyHomeAndZdotdir,

    #[error("RC file not found: {0}")]
    RCFileNotFound(String),
}
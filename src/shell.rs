use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use dirs::{config_dir, home_dir};

use crate::error::ShellError;

pub trait ShellBehavior {
    fn get_rcfiles(&self) -> Result<Vec<PathBuf>, ShellError>;
}

#[derive(Debug)]
/// Represents different types of Unix shells supported by this library.
///
/// This enum provides variants for common Unix shells (POSIX, Zsh, Bash, Fish)
/// along with their specific configuration handling.
///
/// # Examples
///
/// ```
/// use what_the_path::Shell;
///
/// // Detect current shell
/// if let Some(shell) = Shell::detect() {
///     match shell {
///         Shell::Zsh(_) => println!("Using Zsh"),
///         Shell::Bash(_) => println!("Using Bash"),
///         Shell::Fish(_) => println!("Using Fish"),
///         Shell::POSIX(_) => println!("Using POSIX shell"),
///     }
/// }
/// ```
///
/// # Variants
///
/// * `POSIX` - Default POSIX-compliant shell (like sh)
/// * `Zsh` - Z shell
/// * `Bash` - Bourne Again Shell
/// * `Fish` - Friendly Interactive Shell
///
pub enum Shell {
    POSIX(POSIX),
    Zsh(Zsh),
    Bash(Bash),
    Fish(Fish),
}

impl Shell {
    /// Detects the current shell by examining the `SHELL` environment variable.
    ///
    /// This function attempts to identify the shell type based on the `SHELL` environment variable.
    /// It will return `None` on Windows systems as the `SHELL` variable is not typically used.
    ///
    /// # Returns
    /// - `Some(Shell)` containing the detected shell type if:
    ///   - Running on a non-Windows system
    ///   - The `SHELL` environment variable exists and contains a recognized shell name
    /// - `None` if:
    ///   - Running on Windows
    ///   - The `SHELL` environment variable does not exist
    ///
    /// # Shell Detection
    /// The following shells are recognized (in order):
    /// - Zsh
    /// - Bash
    /// - Fish
    /// - Any other shell is assumed to be POSIX-compliant
    pub fn detect_by_shell_var() -> Result<Shell, ShellError> {
        if cfg!(windows) {
            return Err(ShellError::UnsupportedPlatform);
        }

        let shell = env::var("SHELL").map_err(|_| ShellError::NoShellVar)?;

        match shell.as_str() {
            path if path.contains("zsh") => Ok(Shell::Zsh(Zsh)),
            path if path.contains("bash") => Ok(Shell::Bash(Bash)),
            path if path.contains("fish") => Ok(Shell::Fish(Fish)),
            _ => Ok(Shell::POSIX(POSIX)),
        }
    }
}

#[derive(Debug)]
pub struct POSIX;

impl POSIX {
    pub fn does_exist(&self) -> bool {
        true
    }

    pub fn get_rcfiles_from_base(base_dir: impl AsRef<Path>) -> Vec<PathBuf> {
        vec![base_dir.as_ref().join(".profile")]
    }
}

impl ShellBehavior for POSIX {
    fn get_rcfiles(&self) -> Result<Vec<PathBuf>, ShellError> {
        let dir = home_dir().ok_or(ShellError::NoHomeDir)?;
        Ok(vec![dir.join(".profile")])
    }
}

#[derive(Debug)]
pub struct Zsh;

impl Zsh {
    pub fn does_exist(&self) -> bool {
        matches!(env::var("SHELL"), Ok(v) if v.contains("zsh"))
            || Command::new("zsh").output().is_ok()
    }

    pub fn get_rcfiles_from_base(base_dir: impl AsRef<Path>) -> Vec<PathBuf> {
        vec![base_dir.as_ref().join(".zshenv")]
    }
}

impl ShellBehavior for Zsh {
    fn get_rcfiles(&self) -> Result<Vec<PathBuf>, ShellError> {
        let output = std::process::Command::new("zsh")
            .args(["-c", "echo -n $ZDOTDIR"])
            .output()
            .map_err(|_| ShellError::CommandFailed)?;

        if output.stdout.is_empty() {
            return Err(ShellError::EmptyZdotdir);
        }

        // give location
        let location = PathBuf::from(
            String::from_utf8(output.stdout)
                .map_err(|_| ShellError::InvalidUtf8Output)?
                .trim(),
        );
        Ok(vec![location.join(".zshenv")])
    }
}

#[derive(Debug)]
pub struct Bash;

impl Bash {
    pub fn does_exist(&self) -> bool {
        matches!(env::var("SHELL"), Ok(v) if v.contains("bash"))
            || Command::new("bash").output().is_ok()
    }

    pub fn get_rcfiles_from_base(base_dir: impl AsRef<Path>) -> Vec<PathBuf> {
        [".bash_profile", ".bash_login", ".bashrc"]
            .iter()
            .map(|rc| base_dir.as_ref().join(rc))
            .collect()
    }
}

impl ShellBehavior for Bash {
    fn get_rcfiles(&self) -> Result<Vec<PathBuf>, ShellError> {
        let dir = home_dir().ok_or(ShellError::NoHomeDir)?;
        let rcfiles = [".bash_profile", ".bash_login", ".bashrc"]
            .iter()
            .map(|rc| dir.join(rc))
            .collect();
        Ok(rcfiles)
    }
}

#[derive(Debug)]
pub struct Fish;

impl Fish {
    pub fn does_exist(&self) -> bool {
        matches!(env::var("SHELL"), Ok(v) if v.contains("fish"))
            || Command::new("fish").output().is_ok()
    }

    pub fn get_rcfiles_from_base(base_dir: impl AsRef<Path>) -> Vec<PathBuf> {
        vec![base_dir.as_ref().join(".config/fish/conf.d")]
    }
}

impl ShellBehavior for Fish {
    /// Returns the configuration directory path for Fish shell
    ///
    /// This function attempts to locate the Fish shell's configuration directory
    /// by joining the user's home directory with the Fish config path.
    ///
    /// # Returns
    /// - `Some(Vec<PathBuf>)` containing the path to Fish's conf.d directory
    /// - `None` if the home directory cannot be determined
    ///
    /// # Important
    /// Note that this function returns a directory path (`conf.d`), not individual
    /// file paths. You'll need to enumerate the directory contents to access
    /// specific configuration files.
    ///
    /// # Example
    /// ```
    /// if let Some(paths) = get_rcfiles() {
    ///     // paths[0] points to ~/.config/fish/conf.d directory
    ///     // not to specific .fish files
    /// }
    /// ```
    fn get_rcfiles(&self) -> Result<Vec<PathBuf>, ShellError> {
        let mut paths = vec![];

        if let Some(path) = config_dir() {
            paths.push(path.join("fish/conf.d"));
        }

        Ok(paths)
    }
}

pub fn exists_in_path(path: impl AsRef<Path>) -> bool {
    matches!(env::var("PATH"), Ok(paths) if paths.contains(path.as_ref().to_str().unwrap()))
}

pub fn append_to_rcfile(rcfile: PathBuf, line: &str) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new().append(true).open(rcfile)?;
    writeln!(file, "{}", line)
}

pub fn remove_from_rcfile(rcfile: PathBuf, line: &str) -> std::io::Result<()> {
    let line_bytes = line.as_bytes();

    let file = std::fs::read_to_string(&rcfile)?;
    let file_bytes = file.as_bytes();

    if let Some(idx) = file_bytes
        .windows(line_bytes.len())
        .position(|w| w == line_bytes)
    {
        let mut new_bytes = file_bytes[..idx].to_vec();
        new_bytes.extend(&file_bytes[idx + line_bytes.len()..]);
        let content = String::from_utf8(new_bytes).unwrap();
        std::fs::write(&rcfile, content)?;
    }

    Ok(())
}

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::dirs::get_home_dir;

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
    pub fn detect_by_shell_var() -> Option<Shell> {
        if cfg!(windows) {
            return None;
        }

        match env::var("SHELL").ok()?.as_str() {
            shell if shell.contains("zsh") => Some(Shell::Zsh(Zsh)),
            shell if shell.contains("bash") => Some(Shell::Bash(Bash)),
            shell if shell.contains("fish") => Some(Shell::Fish(Fish)),
            _ => Some(Shell::POSIX(POSIX)),
        }
    }
}

#[derive(Debug)]
pub struct POSIX;

impl POSIX {
    pub fn does_exist() -> bool {
        true
    }
    pub fn get_rcfiles() -> Option<Vec<PathBuf>> {
        let dir = get_home_dir()?;
        Some(vec![dir.join(".profile")])
    }
}

#[derive(Debug)]
pub struct Zsh;

impl Zsh {
    pub fn does_exist() -> bool {
        matches!(env::var("SHELL"), Ok(v) if v.contains("zsh"))
            || Command::new("zsh").output().is_ok()
    }

    pub fn get_rcfiles() -> Option<Vec<PathBuf>> {
        let output = std::process::Command::new("zsh")
            .args(["-c", "echo -n $ZDOTDIR"])
            .output()
            .ok()?;

        if output.stdout.is_empty() {
            return None;
        }

        // give location
        let location = PathBuf::from(String::from_utf8(output.stdout).ok()?.trim());
        Some(vec![location.join(".zshenv")])
    }
}

#[derive(Debug)]
pub struct Bash;

impl Bash {
    pub fn does_exist() -> bool {
        matches!(env::var("SHELL"), Ok(v) if v.contains("bash"))
            || Command::new("bash").output().is_ok()
    }

    pub fn get_rcfiles() -> Option<Vec<PathBuf>> {
        let dir = get_home_dir()?;
        let rcfiles = [".bash_profile", ".bash_login", ".bashrc"]
            .iter()
            .map(|rc| dir.join(rc))
            .collect();
        Some(rcfiles)
    }
}

#[derive(Debug)]
pub struct Fish;

impl Fish {
    pub fn does_exist() -> bool {
        matches!(env::var("SHELL"), Ok(v) if v.contains("fish"))
            || Command::new("fish").output().is_ok()
    }

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
    pub fn get_rcfiles() -> Option<Vec<PathBuf>> {
        let mut paths = vec![];

        if let Some(path) = env::var("XDG_CONFIG_HOME").ok() {
            paths.push(PathBuf::from(path).join(".config/fish/conf.d"));
        };

        if let Some(path) = get_home_dir() {
            paths.push(path.join(".config/fish/conf.d"));
        }

        Some(paths)
    }
}

pub fn does_path_exist(path: impl AsRef<Path>) -> bool {
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

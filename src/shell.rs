use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::dirs::get_home_dir;

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

    if let Some(idx) = file_bytes.windows(line_bytes.len()).position(|w| w == line_bytes) {
        let mut new_bytes = file_bytes[..idx].to_vec();
        new_bytes.extend(&file_bytes[idx + line_bytes.len()..]);
        let content = String::from_utf8(new_bytes).unwrap();
        std::fs::write(&rcfile, content)?;
    }

    Ok(())
}

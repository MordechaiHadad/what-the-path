mod dirs;
mod shell;

#[cfg(test)]
mod tests {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    use shell::{does_path_exist, remove_from_rcfile, Bash, Fish, Zsh, POSIX};

    use super::*;

    #[test]
    fn test_does_path_exist() {
        env::set_var("PATH", "/brother:/man");

        assert!(does_path_exist(Path::new("/brother")));
        assert!(does_path_exist(Path::new("/man")));

        assert!(!does_path_exist(Path::new("/nonexistent")));

        env::set_var("PATH", "");
        assert!(!does_path_exist(Path::new("/usr/bin")));
    }

    #[test]
    fn test_posix_get_rcfiles() {
        // Set test home
        env::set_var("HOME", "/home/test");

        let rcfiles = POSIX::get_rcfiles().unwrap();
        assert_eq!(rcfiles.len(), 1);
        assert_eq!(rcfiles[0], PathBuf::from("/home/test/.profile"));
    }

    #[test]
    fn test_bash_get_rcfiles() {
        // Set test home
        env::set_var("HOME", "/home/test");

        let rcfiles = Bash::get_rcfiles().unwrap();
        assert_eq!(rcfiles.len(), 3);
        assert!(rcfiles.contains(&PathBuf::from("/home/test/.bash_profile")));
        assert!(rcfiles.contains(&PathBuf::from("/home/test/.bash_login")));
        assert!(rcfiles.contains(&PathBuf::from("/home/test/.bashrc")));
    }

    #[test]
    fn test_fish_rcfiles() {
        // Test with XDG_CONFIG_HOME
        env::set_var("XDG_CONFIG_HOME", "/custom/xdg");
        let rcfiles = Fish::get_rcfiles().unwrap();
        assert!(rcfiles.contains(&PathBuf::from("/custom/xdg/.config/fish/conf.d")));

        // Test with HOME only
        env::remove_var("XDG_CONFIG_HOME");
        env::set_var("HOME", "/home/test");
        let rcfiles = Fish::get_rcfiles().unwrap();
        
        assert!(rcfiles.contains(&PathBuf::from("/home/test/.config/fish/conf.d")));
    }

    #[test]
    fn test_zsh_rcfiles() {
        // Skip if zsh not available
        if !Zsh::does_exist() {
            return;
        }

        // Test with custom ZDOTDIR
        let test_dir = "/custom/zsh/dir";
        env::set_var("ZDOTDIR", test_dir);
        let rcfiles = Zsh::get_rcfiles().unwrap();
        assert!(rcfiles.contains(&PathBuf::from("/custom/zsh/dir/.zshenv")));

    }

    #[test]
    fn test_rcfiles_with_no_home() {
        // Remove HOME var
        env::remove_var("HOME");

        assert!(POSIX::get_rcfiles().is_none());
        assert!(Bash::get_rcfiles().is_none());
    }
}

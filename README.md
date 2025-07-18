# what-the-path
A Rust library for detecting the current Unix shell, managing shell rc files, and manipulating your PATH.

## Features

- Detect your active shell (Fish, Zsh, Bash, POSIX)
- Locate shell rc files (`.bashrc`, `.zshenv`, `conf.d` for fish, etc.)
- Check if a directory is present in the `PATH` environment variable
- Append or remove lines from shell rc files

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
what-the-path = "0.1"
```

## Usage

```rust
use what_the_path::{Shell, exists_in_path, append_to_rcfile, remove_from_rcfile};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Detect current shell and list rc files
    let shell = Shell::detect_by_shell_var()?;
    let rcfiles = shell.get_rcfiles()?;
    println!("Detected shell: {:?}\nRC files: {:?}", shell, rcfiles);

    // Check if directory is in PATH
    let dir = "/usr/local/bin";
    if exists_in_path(dir) {
        println!("{} is in PATH", dir);
    }

    // Append a line to an rc file
    let rc = PathBuf::from("/home/user/.bashrc");
    append_to_rcfile(rc.clone(), "export FOO=bar")?;

    // Remove a line from an rc file
    remove_from_rcfile(rc, "export FOO=bar")?;

    Ok(())
}
```
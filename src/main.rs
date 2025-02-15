//! Welcome to this Rust workshop. The goal is to create a simple file finder
//! that will print all files matching a given pattern in a given directory
//! and its subdirectories.
//!
//! The first step is to create a program that will list all files in the current
//! directory and its subdirectories.
//!
//! Have fun!

// fn main() {
//     todo!("List all files in the current directory and its subdirectories");
// }

use std::{
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum FileFinderError {
    IoError(std::io::Error),
}

impl std::fmt::Display for FileFinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileFinderError::IoError(error) => write!(f, "I/O error: {error}"),
        }
    }
}

impl From<std::io::Error> for FileFinderError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

type Result<T> = std::result::Result<T, FileFinderError>;

fn iter_files(path: impl AsRef<Path>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let Ok(entry) = entry else {
            continue;
        };

        if entry.file_type()?.is_dir() {
            iter_files(entry.path())?
        } else {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Args {
    dir: PathBuf,
}

fn parse_args() -> Result<Args> {
    let dir = std::env::args()
        .nth(1)
        .map(|dir| PathBuf::from(dir))
        .unwrap_or_else(|| PathBuf::from("."));

    Ok(Args { dir })
}

fn main() -> Result<()> {
    let args: Args = parse_args()?;
    iter_files(args.dir)
}

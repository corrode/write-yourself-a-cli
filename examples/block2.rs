use std::{
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self(err.to_string())
    }
}

type Result<T> = std::result::Result<T, Error>;

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

fn main() -> Result<()> {
    iter_files(PathBuf::from("."))
}

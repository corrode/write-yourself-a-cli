use std::{
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self(format!("{}", err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self(err)
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

#[derive(Debug)]
struct Args {
    root_dir: PathBuf,
}

impl Args {
    fn parse() -> Result<Self> {
        let root_dir = std::env::args()
            .nth(1)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        if !root_dir.exists() {
            return Err(format!("Path does not exist: {}", root_dir.display()).into());
        }
        if !root_dir.is_dir() {
            return Err(format!("Path is not a directory: {}", root_dir.display()).into());
        }
        Ok(Args { root_dir })
    }
}

fn main() {
    if let Err(e) = Args::parse().and_then(|args| iter_files(args.root_dir)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

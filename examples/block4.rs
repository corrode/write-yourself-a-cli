use regex::Regex;
use std::{
    fs::{self},
    path::{Path, PathBuf},
    str::FromStr,
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

fn iter_files(path: impl AsRef<Path>, pattern: &Option<Regex>) -> Result<()> {
    let path = path.as_ref();

    if let Some(p) = pattern {
        // Match the path itself first
        if let Some(path_str) = path.to_str() {
            if p.is_match(path_str) {
                println!("{}", path.display());
            }
        }
    } else {
        println!("{}", path.display());
    }

    // Only try to read directory if the path is a directory
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            iter_files(entry.path(), pattern)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Args {
    root_dir: PathBuf,
    pattern: Option<Regex>,
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

        let pattern = std::env::args()
            .nth(2)
            .map(|s| Regex::from_str(&s).unwrap());

        Ok(Args { root_dir, pattern })
    }
}

fn main() {
    if let Err(e) = Args::parse().and_then(|args| iter_files(args.root_dir, &args.pattern)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

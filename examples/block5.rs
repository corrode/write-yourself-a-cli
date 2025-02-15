use regex::Regex;
use std::{
    collections::VecDeque,
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

struct FileFinder {
    root_dir: PathBuf,
    pattern: Option<Regex>,
}

impl FileFinder {
    fn new(root_dir: impl Into<PathBuf>, pattern: Option<Regex>) -> Self {
        Self {
            root_dir: root_dir.into(),
            pattern,
        }
    }

    fn iter(&self) -> FileFinderIter {
        FileFinderIter {
            pattern: self.pattern.clone(),
            pending: VecDeque::from([self.root_dir.clone()]),
        }
    }
}

struct FileFinderIter {
    pattern: Option<Regex>,
    pending: VecDeque<PathBuf>,
}

impl FileFinderIter {
    fn should_include(&self, path: &Path) -> bool {
        if let Some(ref pattern) = self.pattern {
            path.to_str().map(|s| pattern.is_match(s)).unwrap_or(false)
        } else {
            true
        }
    }
}

impl Iterator for FileFinderIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.pending.pop_front() {
            // If this is a directory, add its contents to the pending queue
            if path.is_dir() {
                if let Ok(entries) = fs::read_dir(&path) {
                    // Add all entries to our pending queue
                    for entry in entries.flatten() {
                        self.pending.push_back(entry.path());
                    }
                }
            }

            // If this path matches our pattern (or we have no pattern), return it
            if self.should_include(&path) {
                return Some(path);
            }
        }

        None
    }
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
    let args = match Args::parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let finder = FileFinder::new(args.root_dir, args.pattern);

    for path in finder.iter() {
        println!("{}", path.display());
    }
}

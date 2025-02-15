use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

fn iter_files(path: PathBuf) {
    for entry in fs::read_dir(path).unwrap() {
        let Ok(entry) = entry else {
            continue;
        };

        let kind = entry.file_type().unwrap();

        if kind.is_dir() {
            iter_files(entry.path())
        } else {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
}

fn main() {
    iter_files(PathBuf::from_str(".").unwrap());
}

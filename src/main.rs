use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};
fn iter_files(path: PathBuf) {
    // Your code here
}
fn main() {
    iter_files(PathBuf::from_str(".").unwrap());
}

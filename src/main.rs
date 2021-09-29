use std::env;
use std::path::PathBuf;

use shorten_path::shorten::{path_to_string, process_path};

fn main() {
    let length = env::var("SHORTHEN_DIR_PATH_LIMIT")
        .map(|x| x.parse::<usize>().unwrap_or(1))
        .unwrap_or(1);
    let home_dir = env::var("HOME")
        .map( PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("$UNDEFINED"));

    for arg in env::args().skip(1).map(PathBuf::from) {
        let path = process_path(&arg, length, &home_dir);
        println!("{}", path_to_string(&path));
    }
}

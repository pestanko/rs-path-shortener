use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let len_str = env::var("SHORTHEN_DIR_PATH_LIMIT").unwrap_or(String::from("1"));
    let length = len_str.parse::<usize>().unwrap_or(1);

    env::args().skip(1).for_each(|arg| {
        let shortened = shorten_path(&arg, length);
        println!("{}", shortened);
    });
}

fn shorten_path(arg: &str, length: usize) -> String {
    let pth = Path::new(arg);
    let parent = pth.parent().unwrap_or(Path::new("."));
    let mut buff = PathBuf::new();
    parent.components().for_each(|component| {
        let name = component.as_os_str().to_str().unwrap_or("");
        let shorten = std::cmp::min(length, name.len());
        buff.push(&name[0..shorten])
    });

    buff.push(pth.file_name().unwrap_or(OsStr::new(".")));
    return buff.to_str().unwrap_or("").to_string();
}

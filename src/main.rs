use std::env;
use std::ffi::{OsStr};
use std::path::Path;
use std::path::PathBuf;

fn main() {
    env::args().skip(1).for_each(|arg| {
        let shortened = shorten_path(&arg, 1);
        println!("{}", shortened);
    });
}

fn shorten_path(arg: &str, length: usize) -> String {
    let pth = Path::new(arg);
    let parent = pth.parent().unwrap_or(Path::new("/"));
    let mut buff = PathBuf::new();
    parent.components().for_each(|component| {
        let name = component.as_os_str().to_str().unwrap_or("");
        let sname = String::from(name);
        let shoten = if length < sname.len() {
            length
        } else {
            sname.len()
        };
        buff.push(&sname[0..shoten])
    });

    buff.push(pth.file_name().unwrap_or(OsStr::new("")));
    return buff.to_str().unwrap_or("").to_string();
}

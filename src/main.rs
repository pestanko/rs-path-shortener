use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let len_str = env::var("SHORTHEN_DIR_PATH_LIMIT").unwrap_or(String::from("1"));
    let home_dir = env::var("HOME")
        .map(|pth| PathBuf::from(pth))
        .unwrap_or(PathBuf::from("$UNDEFINED"));
    let length = len_str.parse::<usize>().unwrap_or(1);

    env::args().skip(1).for_each(|arg| {
        let pth = Path::new(&arg);
        let pth = resolve_home(&pth, &home_dir);
        let shortened = shorten_path(&pth, length);
        println!("{}", shortened.into_os_string().into_string().unwrap());
    });
}

fn shorten_path(pth: &Path, length: usize) -> PathBuf {
    if length == 0 {
        return PathBuf::from(pth);
    }

    let parent = pth.parent().unwrap_or_else(|| {
        if pth.is_absolute() {
            Path::new("/")
        } else {
            Path::new(".")
        }
    });
    let mut buff = PathBuf::new();

    parent.components().for_each(|component| {
        let name = component.as_os_str().to_str().unwrap_or("");
        let shorten = std::cmp::min(length, name.len());
        buff.push(&name[0..shorten])
    });

    buff.push(pth.file_name().unwrap_or_else(|| OsStr::new("")));
    return buff;
}

fn resolve_home(arg: &Path, home_dir: &Path) -> PathBuf {
    if let Ok(striped) = arg.strip_prefix(&home_dir) {
        let mut buf = PathBuf::from("~");
        buf.push(striped);
        buf
    } else {
        PathBuf::from(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOME_PATH: &'static str = "/home/user";

    #[test]
    fn test_resolve_home_not_matching() {
        let home = Path::new(HOME_PATH);
        assert_eq!(
            resolve_home(Path::new("/some/other/path"), home),
            PathBuf::from("/some/other/path")
        );
        assert_eq!(resolve_home(Path::new("/"), home), PathBuf::from("/"));
        assert_eq!(
            resolve_home(Path::new("foo/bar.txt"), home),
            PathBuf::from("foo/bar.txt")
        );
        assert_eq!(
            resolve_home(Path::new("/home/user2/foo.txt"), home),
            PathBuf::from("/home/user2/foo.txt")
        );
    }

    #[test]
    fn test_resolve_home_matching() {
        let home = Path::new(HOME_PATH);
        assert_eq!(resolve_home(Path::new(HOME_PATH), home), PathBuf::from("~"));
        assert_eq!(
            resolve_home(Path::new("/home/user/other"), home),
            PathBuf::from("~/other")
        );
        assert_eq!(
            resolve_home(Path::new("/home/user/other/foo.bar"), home),
            PathBuf::from("~/other/foo.bar")
        );
    }

    #[test]
    fn test_shorten_path_zero_length() {
        assert_eq!(shorten_path(Path::new("/"), 0), PathBuf::from("/"));
        assert_eq!(shorten_path(Path::new("/home"), 0), PathBuf::from("/home"));
        assert_eq!(
            shorten_path(Path::new("/home/user"), 0),
            PathBuf::from("/home/user")
        );
    }

    #[test]
    fn test_shorten_path_one_length() {
        assert_eq!(shorten_path(Path::new("/"), 1), PathBuf::from("/"));
        assert_eq!(shorten_path(Path::new("/home"), 1), PathBuf::from("/home"));
        assert_eq!(shorten_path(Path::new("foo"), 1), PathBuf::from("foo"));
        assert_eq!(
            shorten_path(Path::new("foo/bar"), 1),
            PathBuf::from("f/bar")
        );
        assert_eq!(
            shorten_path(Path::new("/home/user"), 1),
            PathBuf::from("/h/user")
        );
    }

    #[test]
    fn test_shorten_path_hundred_length() {
        assert_eq!(shorten_path(Path::new("/"), 100), PathBuf::from("/"));
        assert_eq!(
            shorten_path(Path::new("/home"), 100),
            PathBuf::from("/home")
        );
        assert_eq!(
            shorten_path(Path::new("/home/user"), 100),
            PathBuf::from("/home/user")
        );
        assert_eq!(shorten_path(Path::new("~"), 100), PathBuf::from("~"));
        assert_eq!(
            shorten_path(Path::new("~/foobar"), 100),
            PathBuf::from("~/foobar")
        );
        assert_eq!(
            shorten_path(Path::new("~/foo.bar"), 100),
            PathBuf::from("~/foo.bar")
        );
        assert_eq!(
            shorten_path(Path::new("~/foo.bar/baz.txt"), 100),
            PathBuf::from("~/foo.bar/baz.txt")
        );
    }
}

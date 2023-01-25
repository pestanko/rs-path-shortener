use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub fn process_path(pth: &Path, length: usize, home_dir: &Path) -> PathBuf {
    let pth = resolve_home(pth, home_dir);
    shorten_path(&pth, length)
}

pub fn shorten_path(pth: &Path, length: usize) -> PathBuf {
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
    buff
}

fn resolve_home(arg: &Path, home_dir: &Path) -> PathBuf {
    if let Ok(striped) = arg.strip_prefix(home_dir) {
        let mut buf = PathBuf::from("~");
        buf.push(striped);
        buf
    } else {
        PathBuf::from(arg)
    }
}

pub fn path_to_string(pth: &Path) -> String {
    pth.display().to_string()
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
        assert_eq!(resolve_home(Path::new("."), home), PathBuf::from("."));
        assert_eq!(
            resolve_home(Path::new("./flex"), home),
            PathBuf::from("./flex")
        );
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
        assert_eq!(
            shorten_path(Path::new("./flex"), 1),
            PathBuf::from("./flex")
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

    #[test]
    fn test_process_path_any() {
        let home = Path::new(HOME_PATH);
        assert_eq!(
            process_path(Path::new("/home/user/foo/bar.txt"), 1, home),
            PathBuf::from("~/f/bar.txt")
        );
        assert_eq!(
            process_path(Path::new("/home/user/docs/foo/bar.txt"), 1, home),
            PathBuf::from("~/d/f/bar.txt")
        );
    }

    #[test]
    fn test_path_to_string() {
        assert_eq!(path_to_string(Path::new("foo.txt")), "foo.txt".to_string());
        assert_eq!(
            path_to_string(Path::new("/foo.txt")),
            "/foo.txt".to_string()
        );
    }
}

use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

pub fn add_file_stem_prefix(path: impl AsRef<Path>, prefix: impl AsRef<OsStr>) -> PathBuf {
    let path = path.as_ref();
    let prefix = prefix.as_ref();

    let stem = path.file_stem().unwrap_or_default();
    let ext = path.extension();
    let ext_len = ext.map_or(0, |e| e.len() + 1);

    let mut new_name = OsString::with_capacity(stem.len() + prefix.len() + ext_len);
    new_name.push(prefix);
    new_name.push(stem);
    if let Some(ext) = ext {
        new_name.push(".");
        new_name.push(ext);
    }

    path.with_file_name(new_name)
}

pub fn add_file_stem_suffix(path: impl AsRef<Path>, suffix: impl AsRef<OsStr>) -> PathBuf {
    let path = path.as_ref();
    let suffix = suffix.as_ref();

    let stem = path.file_stem().unwrap_or_default();
    let ext = path.extension();
    let ext_len = ext.map_or(0, |e| e.len() + 1);

    let mut new_name = OsString::with_capacity(stem.len() + suffix.len() + ext_len);
    new_name.push(stem);
    new_name.push(suffix);
    if let Some(ext) = ext {
        new_name.push(".");
        new_name.push(ext);
    }

    path.with_file_name(new_name)
}

pub trait FileStemExt {
    fn with_added_file_stem_prefix(&self, prefix: impl AsRef<OsStr>) -> PathBuf;
    fn with_added_file_stem_suffix(&self, suffix: impl AsRef<OsStr>) -> PathBuf;
}

impl<T: AsRef<Path>> FileStemExt for T {
    fn with_added_file_stem_prefix(&self, prefix: impl AsRef<OsStr>) -> PathBuf {
        add_file_stem_prefix(self, prefix)
    }
    fn with_added_file_stem_suffix(&self, suffix: impl AsRef<OsStr>) -> PathBuf {
        add_file_stem_suffix(self, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn add_prefix_basic() {
        assert_eq!(
            add_file_stem_prefix(Path::new("foo.txt"), "pre"),
            PathBuf::from("prefoo.txt")
        );
    }

    #[test]
    fn add_suffix_basic() {
        assert_eq!(
            add_file_stem_suffix(Path::new("foo.txt"), "suf"),
            PathBuf::from("foosuf.txt")
        );
    }

    #[test]
    fn no_extension() {
        assert_eq!(
            add_file_stem_prefix(Path::new("foo"), "pre"),
            PathBuf::from("prefoo")
        );
        assert_eq!(
            add_file_stem_suffix(Path::new("foo"), "suf"),
            PathBuf::from("foosuf")
        );
    }

    #[test]
    fn multi_dot_extension() {
        assert_eq!(
            add_file_stem_prefix(Path::new("foo.tar.gz"), "pre"),
            PathBuf::from("prefoo.tar.gz")
        );
        assert_eq!(
            add_file_stem_suffix(Path::new("foo.tar.gz"), "suf"),
            PathBuf::from("foo.tarsuf.gz")
        );
    }

    #[test]
    fn hidden_file() {
        // .gitignore -> stem 含前导点 -> 前缀插在点之前
        assert_eq!(
            add_file_stem_prefix(Path::new(".gitignore"), "pre"),
            PathBuf::from("pre.gitignore")
        );
        assert_eq!(
            add_file_stem_suffix(Path::new(".gitignore"), "suf"),
            PathBuf::from(".gitignoresuf")
        );
        // .config.json -> stem=".config"，ext="json"
        assert_eq!(
            add_file_stem_prefix(Path::new(".config.json"), "pre"),
            PathBuf::from("pre.config.json")
        );
        assert_eq!(
            add_file_stem_suffix(Path::new(".config.json"), "suf"),
            PathBuf::from(".configsuf.json")
        );
    }

    #[test]
    fn with_directory() {
        assert_eq!(
            add_file_stem_prefix(Path::new("dir/foo.txt"), "pre"),
            PathBuf::from("dir/prefoo.txt")
        );
        assert_eq!(
            add_file_stem_suffix(Path::new("dir/foo.txt"), "suf"),
            PathBuf::from("dir/foosuf.txt")
        );
    }

    #[test]
    fn empty_prefix_suffix() {
        assert_eq!(
            add_file_stem_prefix(Path::new("foo.txt"), ""),
            PathBuf::from("foo.txt")
        );
        assert_eq!(
            add_file_stem_suffix(Path::new("foo.txt"), ""),
            PathBuf::from("foo.txt")
        );
    }

    #[test]
    fn prefix_with_dot() {
        assert_eq!(
            add_file_stem_prefix(Path::new("foo.txt"), "pre."),
            PathBuf::from("pre.foo.txt")
        );
    }

    #[test]
    fn trait_methods() {
        let p = Path::new("foo.txt");
        assert_eq!(
            p.with_added_file_stem_prefix("pre"),
            PathBuf::from("prefoo.txt")
        );
        assert_eq!(
            p.with_added_file_stem_suffix("suf"),
            PathBuf::from("foosuf.txt")
        );
    }
}

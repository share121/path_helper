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

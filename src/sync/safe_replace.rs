use crate::sync::gen_unique_path;
use std::{io::Write, path::Path};

/// 安全的替换文件内容。会在同目录下创建一个临时文件，落盘完成后再重命名回原文件。
///
/// # Errors
///
/// IO 操作失败时返回错误
pub fn safe_replace(path: &Path, content: &[u8]) -> std::io::Result<()> {
    let tmp_path = gen_unique_path(path.with_extension("tmp"))?;
    let mut file = std::fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(&tmp_path)?;
    file.write_all(content)?;
    file.sync_all()?;
    std::fs::rename(tmp_path, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[must_use]
    fn tmp_root() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("path_helper_sr_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn creates_new_file() {
        let dir = tmp_root();
        let p = dir.join("new.txt");
        safe_replace(&p, b"hello").unwrap();
        assert_eq!(fs::read(&p).unwrap(), b"hello");
        assert!(!dir.join("new.tmp").exists());
        fs::remove_file(&p).unwrap();
    }

    #[test]
    fn overwrites_existing() {
        let dir = tmp_root();
        let p = dir.join("existing.txt");
        fs::write(&p, b"old").unwrap();
        safe_replace(&p, b"new").unwrap();
        assert_eq!(fs::read(&p).unwrap(), b"new");
        fs::remove_file(&p).unwrap();
    }

    #[test]
    fn empty_content() {
        let dir = tmp_root();
        let p = dir.join("empty.txt");
        safe_replace(&p, b"").unwrap();
        assert_eq!(fs::read(&p).unwrap(), b"");
        fs::remove_file(&p).unwrap();
    }

    #[test]
    fn binary_content() {
        let dir = tmp_root();
        let p = dir.join("bin.dat");
        let data = [0u8, 1, 2, 255, 254, 0, 128];
        safe_replace(&p, &data).unwrap();
        assert_eq!(fs::read(&p).unwrap(), data);
        fs::remove_file(&p).unwrap();
    }

    #[test]
    fn no_extension_file() {
        let dir = tmp_root();
        let p = dir.join("README");
        safe_replace(&p, b"text").unwrap();
        assert_eq!(fs::read(&p).unwrap(), b"text");
        assert!(!dir.join("README.tmp").exists());
        fs::remove_file(&p).unwrap();
    }

    #[test]
    fn parent_missing_errors() {
        let dir = tmp_root().join("nope");
        let p = dir.join("x.txt");
        let err = safe_replace(&p, b"x").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn temp_file_cleaned_up() {
        let dir = tmp_root();
        let p = dir.join("replace.txt");
        fs::write(&p, b"orig").unwrap();
        safe_replace(&p, b"updated").unwrap();
        assert!(!dir.join("replace.tmp").exists());
        fs::remove_file(&p).unwrap();
    }
}

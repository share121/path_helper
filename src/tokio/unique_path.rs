use std::path::{Path, PathBuf};

/// 生成一个唯一的路径，若路径已存在则在文件名后加上 (序号)
///
/// 如 `example.zip` 会变成 `example (1).zip`
///
/// 使用 `create_new` 原子性占位，消除高并发下的 TOCTOU 竞态条件。
/// 返回的路径对应一个已创建的空文件，调用者可直接写入。
///
/// # Errors
/// 当 IO 操作（创建文件、检查父目录等）失败时返回 Error
pub async fn gen_unique_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let path = path.as_ref();

    let mut open_option = tokio::fs::OpenOptions::new();
    open_option.create_new(true).write(true);

    match open_option.open(path).await {
        Ok(_) => return Ok(path.to_path_buf()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(e),
    }

    let stem = path.file_stem().unwrap_or_default();
    let ext = path.extension();
    for i in 1.. {
        let mut new_name = stem.to_os_string();
        new_name.push(" (");
        new_name.push(i.to_string());
        if let Some(ext) = ext {
            new_name.push(").");
            new_name.push(ext);
        } else {
            new_name.push(")");
        }
        let new_path = path.with_file_name(new_name);
        match open_option.open(&new_path).await {
            Ok(_) => return Ok(new_path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    unreachable!("loop should always find a free filename or return an error")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[must_use]
    fn tmp_root() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("path_helper_tu_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test]
    async fn creates_new_file() {
        let dir = tmp_root();
        let target = dir.join("new.txt");
        let got = gen_unique_path(&target).await.unwrap();
        assert_eq!(got, target);
        assert!(got.exists());
        assert_eq!(fs::metadata(&got).unwrap().len(), 0);
        fs::remove_file(&got).unwrap();
    }

    #[tokio::test]
    async fn existing_gets_numbered() {
        let dir = tmp_root();
        let target = dir.join("a.zip");
        fs::write(&target, b"orig").unwrap();
        let got = gen_unique_path(&target).await.unwrap();
        assert_eq!(got, dir.join("a (1).zip"));
        fs::remove_file(&target).unwrap();
        fs::remove_file(&got).unwrap();
    }

    #[tokio::test]
    async fn parent_missing_errors() {
        let dir = tmp_root().join("nope/sub");
        let target = dir.join("x.txt");
        let err = gen_unique_path(&target).await.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}

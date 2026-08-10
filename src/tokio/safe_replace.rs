use crate::tokio::gen_unique_path;
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// 安全的替换文件内容。会在同目录下创建一个临时文件，落盘完成后再重命名回原文件。
///
/// # Errors
///
/// IO 操作失败时返回错误
pub async fn safe_replace(path: impl AsRef<Path>, content: &[u8]) -> std::io::Result<()> {
    let path = path.as_ref();
    let tmp_path = gen_unique_path(path.with_extension("tmp")).await?;
    let mut file = tokio::fs::OpenOptions::new()
        .truncate(true)
        .create(false)
        .write(true)
        .open(&tmp_path)
        .await?;
    file.write_all(content).await?;
    file.sync_all().await?;
    tokio::fs::rename(tmp_path, path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[must_use]
    fn tmp_root() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("path_helper_ts_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test]
    async fn creates_new_file() {
        let dir = tmp_root();
        let p = dir.join("new.txt");
        safe_replace(&p, b"hello").await.unwrap();
        assert_eq!(fs::read(&p).unwrap(), b"hello");
        assert!(!dir.join("new.tmp").exists());
        fs::remove_file(&p).unwrap();
    }

    #[tokio::test]
    async fn overwrites_existing() {
        let dir = tmp_root();
        let p = dir.join("existing.txt");
        fs::write(&p, b"old").unwrap();
        safe_replace(&p, b"new").await.unwrap();
        assert_eq!(fs::read(&p).unwrap(), b"new");
        fs::remove_file(&p).unwrap();
    }

    #[tokio::test]
    async fn parent_missing_errors() {
        let dir = tmp_root().join("nope");
        let p = dir.join("x.txt");
        let err = safe_replace(&p, b"x").await.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}

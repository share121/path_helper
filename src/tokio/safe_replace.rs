use crate::tokio::gen_unique_path;
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// 安全的替换文件内容。会在同目录下创建一个临时文件，落盘完成后再重命名回原文件。
pub async fn safe_replace(path: &Path, content: &[u8]) -> std::io::Result<()> {
    let tmp_path = gen_unique_path(path.with_extension("tmp")).await?;
    let mut file = tokio::fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(&tmp_path)
        .await?;
    file.write_all(content).await?;
    file.sync_all().await?;
    tokio::fs::rename(tmp_path, path).await?;
    Ok(())
}

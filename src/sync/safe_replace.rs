use crate::sync::gen_unique_path;
use std::{io::Write, path::Path};

/// 安全的替换文件内容。会在同目录下创建一个临时文件，落盘完成后再重命名回原文件。
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

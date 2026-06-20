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

    match tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .await
    {
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
        match tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&new_path)
            .await
        {
            Ok(_) => return Ok(new_path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    unreachable!("loop should always find a free filename or return an error")
}

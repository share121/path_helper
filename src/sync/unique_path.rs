use std::path::{Path, PathBuf};

/// 生成一个唯一的路径，若路径已存在则在文件名后加上 (序号)
///
/// 如 `example.zip` 会变成 `example (1).zip`
///
/// # Errors
/// 当 `std::fs::exists` 失败时返回 Error
pub fn gen_unique_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    if !std::fs::exists(path)? {
        return Ok(path.into());
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
        if !std::fs::exists(&new_path)? {
            return Ok(new_path);
        }
    }
    unreachable!()
}

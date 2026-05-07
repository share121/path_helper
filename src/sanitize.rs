use crate::{is_extension, truncate_filename};
use std::path::{Component, Path, PathBuf};

/// 对文件名进行安全处理，保留扩展名，截断过长的文件名部分
pub fn sanitize_filename(filename: impl AsRef<str>, max_units: usize) -> String {
    let filename = filename.as_ref();
    let options = sanitize_filename::Options {
        windows: cfg!(windows),
        truncate: false,
        replacement: "_",
    };
    let cleaned = sanitize_filename::sanitize_with_options(filename, options);
    let (base, ext) = if let Some(pos) = cleaned.rfind('.') {
        let ext_candidate = &cleaned[pos..];
        if is_extension(ext_candidate) {
            (&cleaned[..pos], ext_candidate)
        } else {
            (&cleaned[..], "")
        }
    } else {
        (&cleaned[..], "")
    };
    let final_base = truncate_filename(base, ext, max_units);
    format!("{}{}", final_base, ext)
}

/// 对路径进行安全处理，保留扩展名，截断过长的文件名部分
pub fn sanitize_path(path: &Path) -> PathBuf {
    path.components()
        .map(|c| match c {
            Component::Normal(name) => {
                let s = name.to_string_lossy();
                sanitize_filename(&s, 255).into()
            }
            Component::ParentDir => "..".into(),
            Component::CurDir => ".".into(),
            Component::RootDir => std::path::MAIN_SEPARATOR_STR.into(),
            Component::Prefix(prefix) => prefix.as_os_str().to_os_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    fn assert_length(s: &str, max_units: usize) {
        let units = s.encode_utf16().count();
        assert!(
            units <= max_units,
            "Windows: length {} exceeded {}",
            units,
            max_units
        );
    }
    #[cfg(unix)]
    fn assert_length(s: &str, max_units: usize) {
        let units = s.len();
        assert!(
            units <= max_units,
            "Unix: length {} exceeded {}",
            units,
            max_units
        );
    }

    #[test]
    fn test_sanitize() {
        // 文件名：file_stem.ext
        // 测试长文件名保留后缀（当 ext 较短时优先截断 file_stem）
        let long_stem = "这是一个非常".repeat(50);
        let long_name = format!("{}.mp4", long_stem);
        let result = sanitize_filename(&long_name, 255);
        assert!(result.ends_with(".mp4"));
        assert_length(&result, 255);

        // 文件名：file_stem.ext
        // 测试非常长的后缀名（当 ext 过长时，可能他并没有扩展名，类似“1.这是第一个标题”，显然“这是第一个标题”不是文件后缀名，因此 file_stem.ext 当成整个文件名截断
        let long_stem = "这是一个非常".repeat(50);
        let long_name = format!("1.{}", long_stem);
        let result = sanitize_filename(&long_name, 255);
        assert_length(&result, 255);

        // 测试普通后缀
        let normal_name = "我的文件.test.txt";
        let result = sanitize_filename(normal_name, 255);
        assert_eq!(result, "我的文件.test.txt");

        // 测试包含非法字符
        let illegal = "test/\\:*?\"<>|.png";
        let result = sanitize_filename(illegal, 255);
        assert_eq!(result, "test_________.png");
    }
}

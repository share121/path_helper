use crate::{is_extension, truncate_filename};
use std::path::{Component, Path, PathBuf};

/// 对文件名进行安全处理，保留扩展名，截断过长的文件名部分。
/// 结果总长度（含扩展名）不超过 `max_units`；若扩展名本身已超出预算则一并截断。
pub fn sanitize_filename(filename: impl AsRef<str>, max_units: usize) -> String {
    let filename = filename.as_ref();
    let options = sanitize_filename::Options {
        windows: cfg!(windows),
        truncate: false,
        replacement: "_",
    };
    let cleaned = sanitize_filename::sanitize_with_options(filename, options);
    let (base, ext) = cleaned.rfind('.').map_or_else(
        || (&cleaned[..], ""),
        |pos| {
            let ext_candidate = &cleaned[pos..];
            if is_extension(ext_candidate) {
                (&cleaned[..pos], ext_candidate)
            } else {
                (&cleaned[..], "")
            }
        },
    );
    truncate_filename(base, ext, max_units).into_owned()
}

/// 对路径进行安全处理，保留扩展名，截断过长的文件名部分
#[must_use]
pub fn sanitize_path(path: &Path) -> PathBuf {
    let mut buf = PathBuf::with_capacity(path.as_os_str().len());
    for c in path.components() {
        match c {
            Component::Prefix(p) => buf.push(p.as_os_str()),
            Component::RootDir => buf.push(std::path::MAIN_SEPARATOR_STR),
            Component::CurDir => buf.push("."),
            Component::ParentDir => buf.push(".."),
            Component::Normal(name) => {
                let s = name.to_string_lossy();
                buf.push(sanitize_filename(&s, 255));
            }
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    fn assert_length(s: &str, max_units: usize) {
        let units = s.encode_utf16().count();
        assert!(
            units <= max_units,
            "Windows: length {units} exceeded {max_units}",
        );
    }
    #[cfg(unix)]
    fn assert_length(s: &str, max_units: usize) {
        let units = s.len();
        assert!(
            units <= max_units,
            "Unix: length {units} exceeded {max_units}",
        );
    }

    #[must_use]
    fn norm(p: &Path) -> String {
        p.to_string_lossy().replace('\\', "/")
    }

    #[test]
    fn sanitize_replaces_illegal_chars() {
        assert_eq!(
            sanitize_filename("test/\\:*?\"<>|.png", 255),
            "test_________.png"
        );
        assert_eq!(sanitize_filename("a*b", 255), "a_b");
        assert_eq!(sanitize_filename("a?b", 255), "a_b");
    }

    #[test]
    fn sanitize_keeps_extension_and_truncates_long_stem() {
        let long_stem = "这是一个非常".repeat(50);
        let long_name = format!("{long_stem}.mp4");
        let result = sanitize_filename(&long_name, 255);
        assert!(Path::new(&result)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("mp4")));
        assert_length(&result, 255);
    }

    #[test]
    fn sanitize_long_pseudo_extension_truncates_whole() {
        let long_stem = "这是一个非常".repeat(50);
        let long_name = format!("1.{long_stem}");
        let result = sanitize_filename(&long_name, 255);
        assert_length(&result, 255);
    }

    #[test]
    fn sanitize_normal_multidot_extension() {
        assert_eq!(
            sanitize_filename("我的文件.test.txt", 255),
            "我的文件.test.txt"
        );
        assert_eq!(sanitize_filename("foo.bar.baz", 255), "foo.bar.baz");
    }

    #[test]
    fn sanitize_no_extension() {
        assert_eq!(sanitize_filename("我的文件", 255), "我的文件");
        assert_eq!(sanitize_filename("plain", 255), "plain");
    }

    #[test]
    fn sanitize_empty() {
        let result = sanitize_filename("", 255);
        assert_length(&result, 255);
        assert_eq!(result, "");
    }

    #[test]
    fn sanitize_hidden_file() {
        // 单点隐藏文件被整体当扩展名（stem 空），字串恰好不变
        assert_eq!(sanitize_filename(".gitignore", 255), ".gitignore");
        assert_eq!(sanitize_filename(".myhidden", 255), ".myhidden");
    }

    #[test]
    fn sanitize_respects_small_max_units() {
        let result = sanitize_filename("abcdefghij.txt", 12);
        assert!(Path::new(&result)
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("txt")));
        assert_length(&result, 12);
    }

    #[test]
    fn sanitize_path_basic() {
        assert_eq!(
            norm(&sanitize_path(Path::new("foo/bar.txt"))),
            "foo/bar.txt"
        );
    }

    #[test]
    fn sanitize_path_illegal_in_component() {
        let result = sanitize_path(Path::new("a/b:*?/c.png"));
        let s = norm(&result);
        assert!(!s.contains(':'));
        assert!(!s.contains('*'));
    }

    #[test]
    fn sanitize_path_preserves_parent_dir() {
        let result = sanitize_path(Path::new("a/../b"));
        let comps: Vec<String> = result
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();
        assert_eq!(comps, vec!["a", "..", "b"]);
    }

    #[test]
    fn sanitize_path_long_component_truncated() {
        let long_stem = "这是一个非常".repeat(50);
        let name = format!("{long_stem}.mp4");
        let p = Path::new(&name);
        let result = sanitize_path(p);
        assert_length(&norm(&result), 255);
        assert!(result
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("mp4")));
    }

    #[test]
    fn sanitize_path_empty() {
        assert_eq!(norm(&sanitize_path(Path::new(""))), "");
    }

    #[cfg(windows)]
    #[test]
    fn sanitize_path_windows_prefix() {
        let result = sanitize_path(Path::new("C:\\foo\\bar.txt"));
        let s = norm(&result);
        assert!(s.starts_with("C:"));
        assert!(s.ends_with("bar.txt"));
    }

    #[cfg(unix)]
    #[test]
    fn sanitize_path_unix_absolute() {
        let result = sanitize_path(Path::new("/foo/bar.txt"));
        let s = norm(&result);
        assert!(s.starts_with('/'));
        assert!(s.ends_with("bar.txt"));
    }
}

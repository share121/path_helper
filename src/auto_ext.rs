use crate::is_extension;
use std::borrow::Cow;

fn best_ext(content_type: &str) -> Option<&str> {
    let mime_type = content_type.split(';').next().map(str::trim)?;
    match mime_type {
        "application/octet-stream" => None,
        _ => mime_guess::get_mime_extensions_str(mime_type).and_then(min_ext),
    }
}

fn min_ext<'a>(extensions: &[&'a str]) -> Option<&'a str> {
    extensions
        .iter()
        .copied()
        .min_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)))
}

/// 根据文件名和 Content-Type 自动添加扩展名
#[must_use]
pub fn auto_ext<'a>(file_name: &'a str, content_type: Option<&str>) -> Cow<'a, str> {
    let file_name = file_name.trim_end_matches('.');
    let has_valid_ext = file_name
        .rfind('.')
        .is_some_and(|pos| is_extension(&file_name[pos + 1..]));
    if has_valid_ext {
        return Cow::Borrowed(file_name);
    }
    if let Some(ct) = content_type {
        if let Some(ext) = best_ext(ct) {
            return Cow::Owned(format!("{file_name}.{ext}"));
        }
    }
    Cow::Borrowed(file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_extension_is_kept_and_content_type_ignored() {
        assert_eq!(
            "document.pdf",
            auto_ext("document.pdf", Some("application/pdf"))
        );
        // 已有合法扩展名时，即使 CT 不匹配也应原样返回
        assert_eq!("a.txt", auto_ext("a.txt", Some("image/png")));
    }

    #[test]
    fn appends_shortest_extension_from_content_type() {
        assert_eq!("avatar.jpe", auto_ext("avatar", Some("image/jpeg")));
        assert_eq!(
            "index.htm",
            auto_ext("index", Some("text/html; charset=utf-8"))
        );
        assert_eq!(
            "data.json",
            auto_ext("data", Some("application/json; charset=utf-8; boundary=x"))
        );
    }

    #[test]
    fn no_content_type_returns_borrowed() {
        assert_eq!("unknown_file", auto_ext("unknown_file", None));
        assert_eq!("视频.mp4", auto_ext("视频.mp4", None));
    }

    #[test]
    fn invalid_content_type_returns_borrowed() {
        assert_eq!("some_file", auto_ext("some_file", Some("not-a-valid-mime")));
    }

    #[test]
    fn octet_stream_adds_no_extension() {
        assert_eq!("视频", auto_ext("视频", Some("application/octet-stream")));
        assert_eq!("file", auto_ext("file", Some("application/octet-stream")));
    }

    #[test]
    fn hidden_file_treated_as_having_extension() {
        assert_eq!(".gitignore", auto_ext(".gitignore", Some("text/plain")));
        assert_eq!(".gitignore", auto_ext(".gitignore", None));
    }

    #[test]
    fn multi_dot_uses_last_segment() {
        assert_eq!(
            "archive.tar.gz",
            auto_ext("archive.tar.gz", Some("application/gzip"))
        );
        assert_eq!("archive.tar.gz", auto_ext("archive.tar.gz", None));
    }

    #[test]
    fn chinese_filename() {
        assert_eq!(
            "1.这是一个视频.mp4",
            auto_ext("1.这是一个视频", Some("video/mp4"))
        );
        assert_eq!("视频.mp4", auto_ext("视频", Some("video/mp4")));
    }

    #[test]
    fn empty_and_dotty_filenames() {
        assert_eq!("", auto_ext("", None));
        assert_eq!(".png", auto_ext("", Some("image/png")));
        assert_eq!(".png", auto_ext("...", Some("image/png")));
        assert_eq!("", auto_ext(".", None));
        assert_eq!("file.png", auto_ext("file.", Some("image/png")));
    }

    #[test]
    fn content_type_is_case_insensitive() {
        assert_eq!("img.jpe", auto_ext("img", Some("IMAGE/JPEG")));
        assert_eq!("img.jpe", auto_ext("img", Some("Image/Jpeg")));
        assert_eq!("img.png", auto_ext("img", Some("IMAGE/PNG")));
    }

    #[test]
    fn min_ext_picks_shortest_then_lexicographic() {
        assert_eq!(min_ext(&["jpeg", "jpg", "jpe"]), Some("jpe"));
        assert_eq!(min_ext(&["html", "htm"]), Some("htm"));
        assert_eq!(min_ext(&["zzz", "aaa"]), Some("aaa"));
        assert_eq!(min_ext(&[]), None);
        assert_eq!(min_ext(&["only"]), Some("only"));
    }

    #[test]
    fn best_ext_handles_content_type() {
        assert_eq!(best_ext("application/octet-stream"), None);
        assert_eq!(best_ext("image/jpeg"), Some("jpe"));
        assert_eq!(best_ext("text/html; charset=utf-8"), Some("htm"));
        assert_eq!(best_ext(" text/html "), Some("htm"));
        assert_eq!(best_ext(""), None);
        assert_eq!(best_ext("not-a-mime"), None);
        assert_eq!(best_ext("IMAGE/PNG"), Some("png"));
    }
}

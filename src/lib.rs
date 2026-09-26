#[cfg(feature = "auto_ext")]
mod auto_ext;
#[cfg(feature = "auto_ext")]
pub use auto_ext::*;

#[cfg(feature = "sanitize")]
mod sanitize;
#[cfg(feature = "sanitize")]
pub use sanitize::*;

#[cfg(feature = "tokio")]
pub mod tokio;

mod sync;
pub use sync::*;

mod file_stem;
pub use file_stem::*;

mod iter_stem;
pub use iter_stem::*;

mod truncate;
pub use truncate::*;

mod comparable;
pub use comparable::*;

/// 检查扩展名是否合法
#[must_use]
pub fn is_extension(mut ext: &str) -> bool {
    ext = ext.trim_start_matches('.');
    !ext.is_empty() && ext.len() <= 16 && ext.chars().all(|c| c.is_ascii_graphic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_extensions() {
        assert!(is_extension("txt"));
        assert!(is_extension(".txt"));
        assert!(is_extension("tar.gz"));
        assert!(is_extension("pdf"));
        assert!(is_extension("a"));
    }

    #[test]
    fn leading_dots_are_trimmed() {
        assert!(is_extension("..txt"));
        assert!(is_extension("...ext"));
    }

    #[test]
    fn rejects_empty_and_only_dots() {
        assert!(!is_extension(""));
        assert!(!is_extension("."));
        assert!(!is_extension(".."));
        assert!(!is_extension("..."));
    }

    #[test]
    fn length_boundary() {
        let s16 = "a".repeat(16);
        let s17 = "a".repeat(17);
        assert!(is_extension(&s16));
        assert!(!is_extension(&s17));
    }

    #[test]
    fn leading_dot_does_not_count_toward_length() {
        let with16 = format!(".{}", "a".repeat(16));
        let with17 = format!(".{}", "a".repeat(17));
        assert!(is_extension(&with16));
        assert!(!is_extension(&with17));
    }

    #[test]
    fn rejects_non_ascii() {
        assert!(!is_extension("中文"));
        assert!(!is_extension("txt中文"));
        assert!(!is_extension("é"));
    }

    #[test]
    fn ascii_graphic_but_filesystem_illegal_is_accepted() {
        // is_extension 只校验 "ASCII graphic"，不拒绝文件名非法字符
        assert!(is_extension("a/b"));
        assert!(is_extension("a:b"));
        assert!(is_extension("a*b"));
        assert!(is_extension("a?b"));
        assert!(is_extension("a<b"));
        assert!(is_extension("a>b"));
    }

    #[test]
    fn rejects_whitespace() {
        assert!(!is_extension("txt "));
        assert!(!is_extension(" txt"));
    }
}

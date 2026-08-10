use std::borrow::Cow;

#[cfg(windows)]
#[must_use]
/// 截断 `base` 使 `base` + `ext` 的总长度（UTF-16 单元）不超过 `max_units`。
/// 若 `ext` 自身已占满或超出预算，则连 `ext` 也一并截断，保证总长 ≤ `max_units` 恒成立。
pub fn truncate_filename<'a>(base: &'a str, ext: &'a str, max_units: usize) -> Cow<'a, str> {
    let ext_units = ext.encode_utf16().count();
    if ext_units >= max_units {
        return Cow::Borrowed(truncate_units(ext, max_units));
    }
    let max_base_units = max_units - ext_units;
    Cow::Owned(format!("{}{ext}", truncate_units(base, max_base_units)))
}

#[cfg(unix)]
#[must_use]
/// 截断 `base` 使 `base` + `ext` 的总长度（字节）不超过 `max_units`。
/// 若 `ext` 自身已占满或超出预算，则连 `ext` 也一并截断，保证总长 ≤ `max_units` 恒成立。
pub fn truncate_filename<'a>(base: &'a str, ext: &'a str, max_units: usize) -> Cow<'a, str> {
    let ext_bytes = ext.len();
    if ext_bytes >= max_units {
        return Cow::Borrowed(truncate_units(ext, max_units));
    }
    let max_base_bytes = max_units - ext_bytes;
    Cow::Owned(format!("{}{ext}", truncate_units(base, max_base_bytes)))
}

#[cfg(windows)]
fn truncate_units(s: &str, max_units: usize) -> &str {
    let mut units = 0;
    let mut byte_index = 0;
    for (i, c) in s.char_indices() {
        let u = c.len_utf16();
        if units + u > max_units {
            break;
        }
        units += u;
        byte_index = i + c.len_utf8();
    }
    &s[..byte_index]
}

#[cfg(unix)]
fn truncate_units(s: &str, max_units: usize) -> &str {
    if s.len() <= max_units {
        return s;
    }
    let mut end = max_units;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[must_use]
    fn units(s: &str) -> usize {
        s.encode_utf16().count()
    }
    #[cfg(unix)]
    #[must_use]
    fn units(s: &str) -> usize {
        s.len()
    }

    #[test]
    fn result_base_part_is_prefix_of_base() {
        // 非退化输入下，去掉尾部 ext 后应是 base 的前缀
        let base = "Hello世界-test";
        for max in [4usize, 8, 20, 255] {
            let r = &*truncate_filename(base, ".txt", max);
            let base_part = r.strip_suffix(".txt").unwrap_or(r);
            assert!(base.starts_with(base_part), "max={max}");
        }
    }

    #[test]
    fn total_length_never_exceeds_max() {
        let base = "这是一个测试字符串long";
        // 扩展名也会被截断，因此即使 max < ext 长度，总长也保证 <= max
        for max in [1usize, 2, 3, 4, 5, 12, 30, 255] {
            let r = &*truncate_filename(base, ".mp4", max);
            let total = units(r);
            assert!(total <= max, "total {total} > max {max}");
        }
    }

    #[test]
    fn fits_whole_when_room() {
        assert_eq!(&*truncate_filename("abc", "", 10), "abc");
        assert_eq!(&*truncate_filename("abc", ".txt", 255), "abc.txt");
    }

    #[test]
    fn empty_inputs() {
        assert_eq!(&*truncate_filename("", "", 255), "");
        assert_eq!(&*truncate_filename("", ".txt", 255), ".txt");
    }

    #[test]
    fn max_units_zero_yields_truncated_ext() {
        // ext 自身 >= max_units 时连扩展名一起截，结果 = ext 前缀
        assert_eq!(&*truncate_filename("abc", ".txt", 2), ".t");
    }

    #[cfg(windows)]
    #[test]
    fn windows_exact_units() {
        // 10 个 BMP 汉字 = 10 UTF-16 单元；ext ".txt"=4；max=12 -> max_base=8 -> base 8 单元 + ext
        assert_eq!(
            &*truncate_filename("一二三四五六七八九十", ".txt", 12),
            "一二三四五六七八.txt"
        );
        assert_eq!(&*truncate_filename("AB", "", 2), "AB");
        assert_eq!(&*truncate_filename("ABC", "", 2), "AB");
    }

    #[cfg(unix)]
    #[test]
    fn unix_exact_bytes() {
        // 5 个 CJK = 15 字节；ext ".txt"=4；max=12 -> max_base=8 -> 8 字节 = 2 CJK + ext
        assert_eq!(&*truncate_filename("一二三四五六", ".txt", 12), "一二.txt");
        assert_eq!(&*truncate_filename("AB", "", 2), "AB");
        assert_eq!(&*truncate_filename("ABC", "", 2), "AB");
    }
}

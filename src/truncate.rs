#[cfg(windows)]
#[must_use]
pub fn truncate_filename<'a>(base: &'a str, ext: &str, max_units: usize) -> &'a str {
    let ext_units = ext.encode_utf16().count();
    let max_base_units = max_units.saturating_sub(ext_units);
    let mut current_units = 0;
    let mut byte_index = 0;
    for (i, c) in base.char_indices() {
        let units = c.len_utf16();
        if current_units + units > max_base_units {
            break;
        }
        current_units += units;
        byte_index = i + c.len_utf8();
    }
    &base[..byte_index]
}

#[cfg(unix)]
#[must_use]
pub fn truncate_filename<'a>(base: &'a str, ext: &str, max_units: usize) -> &'a str {
    let ext_bytes = ext.len();
    let max_base_bytes = max_units.saturating_sub(ext_bytes);
    if base.len() <= max_base_bytes {
        return base;
    }
    let mut end = max_base_bytes;
    while end > 0 && !base.is_char_boundary(end) {
        end -= 1;
    }
    &base[..end]
}

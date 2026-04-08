#[cfg(feature = "auto_ext")]
mod auto_ext;
#[cfg(feature = "sanitize")]
mod sanitize;
mod unique_path;

#[cfg(feature = "auto_ext")]
pub use auto_ext::*;
#[cfg(feature = "sanitize")]
pub use sanitize::*;
pub use unique_path::*;

/// 检查扩展名是否合法
pub fn is_extension(ext: &str) -> bool {
    !ext.is_empty() && ext.len() <= 16 && ext.chars().all(|c| c.is_ascii_graphic())
}

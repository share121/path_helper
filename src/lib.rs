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

/// 检查扩展名是否合法
#[must_use]
pub fn is_extension(mut ext: &str) -> bool {
    ext = ext.trim_start_matches('.');
    !ext.is_empty() && ext.len() <= 16 && ext.chars().all(|c| c.is_ascii_graphic())
}

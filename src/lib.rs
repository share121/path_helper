#[cfg(feature = "auto_ext")]
mod auto_ext;
mod file_stem;
#[cfg(feature = "sanitize")]
mod sanitize;
mod sync;
#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(feature = "auto_ext")]
pub use auto_ext::*;
#[cfg(feature = "sanitize")]
pub use sanitize::*;
pub use sync::*;

pub use file_stem::*;

/// 检查扩展名是否合法
#[must_use]
pub fn is_extension(mut ext: &str) -> bool {
    ext = ext.trim_start_matches('.');
    !ext.is_empty() && ext.len() <= 16 && ext.chars().all(|c| c.is_ascii_graphic())
}

mod truncate;
pub use truncate::*;

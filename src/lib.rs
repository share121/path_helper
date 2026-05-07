#[cfg(feature = "auto_ext")]
mod auto_ext;
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

/// 检查扩展名是否合法
pub fn is_extension(ext: &str) -> bool {
    !ext.is_empty() && ext.len() <= 16 && ext.chars().all(|c| c.is_ascii_graphic())
}

mod truncate;
pub use truncate::*;

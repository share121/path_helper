# `path_helper`

[![GitHub last commit](https://img.shields.io/github/last-commit/share121/path_helper/main)](https://github.com/share121/path_helper/commits/main)
[![Test](https://github.com/share121/path_helper/workflows/Test/badge.svg)](https://github.com/share121/path_helper/actions)
[![Latest version](https://img.shields.io/crates/v/path_helper.svg)](https://crates.io/crates/path_helper)
[![Documentation](https://docs.rs/path_helper/badge.svg)](https://docs.rs/path_helper)
[![License](https://img.shields.io/crates/l/path_helper.svg)](https://github.com/share121/path_helper/blob/main/LICENSE)

简单易用的路径处理库

## 功能

1. `gen_unique_path`: 自动给路径加 (序号)，如 `example.zip` 会变成 `example (1).zip`
2. `sanitize_filename`: 规范化文件名，文件名过长时会自动截断，并且保留扩展名
3. `sanitize_path`: 规范化路径中的每一个文件(夹)名，并且保留扩展名

```rust
use file_alloc::{init_fast_alloc, FileAlloc};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_fast_alloc(); // 必须在打开文件前调用，否则无法享受快分配

    let temp_file = tempfile::NamedTempFile::new()?;
    let mut file = tokio::fs::File::options()
        .read(true)
        .write(true)
        .open(temp_file.path())
        .await?;

    let target_size = 5 * 1024 * 1024;
    file.allocate(target_size).await?; // 文件预分配

    let metadata = file.metadata().await?;
    assert_eq!(metadata.len(), target_size);
    Ok(())
}
```

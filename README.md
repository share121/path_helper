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

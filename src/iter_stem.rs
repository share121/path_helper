use crate::FileStemExt;
use std::path::{Path, PathBuf};

pub struct IterStem {
    path: PathBuf,
    i: usize,
}

impl Iterator for IterStem {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.i == 0 {
            self.path.clone()
        } else {
            self.path
                .with_added_file_stem_suffix(format!(" ({})", self.i))
        };
        self.i += 1;
        Some(res)
    }
}

pub trait IterStemExt {
    fn iter_stem(&self) -> IterStem;
}

impl<P: AsRef<Path>> IterStemExt for P {
    fn iter_stem(&self) -> IterStem {
        IterStem {
            path: self.as_ref().to_path_buf(),
            i: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn yields_original_then_numbered() {
        let v: Vec<PathBuf> = Path::new("foo.txt").iter_stem().take(3).collect();
        assert_eq!(
            v,
            vec![
                PathBuf::from("foo.txt"),
                PathBuf::from("foo (1).txt"),
                PathBuf::from("foo (2).txt"),
            ]
        );
    }

    #[test]
    fn no_extension() {
        let v: Vec<PathBuf> = Path::new("foo").iter_stem().take(3).collect();
        assert_eq!(
            v,
            vec![
                PathBuf::from("foo"),
                PathBuf::from("foo (1)"),
                PathBuf::from("foo (2)"),
            ]
        );
    }

    #[test]
    fn hidden_file() {
        let v: Vec<PathBuf> = Path::new(".gitignore").iter_stem().take(2).collect();
        assert_eq!(
            v,
            vec![PathBuf::from(".gitignore"), PathBuf::from(".gitignore (1)")]
        );
    }

    #[test]
    fn multi_dot_extension() {
        let v: Vec<PathBuf> = Path::new("foo.tar.gz").iter_stem().take(2).collect();
        assert_eq!(
            v,
            vec![PathBuf::from("foo.tar.gz"), PathBuf::from("foo.tar (1).gz")]
        );
    }

    #[test]
    fn is_infinite_iterator() {
        // 取很多个也不应停止（调用方必须用 take / 自行截断）
        let count = Path::new("a.txt").iter_stem().take(1000).count();
        assert_eq!(count, 1000);
    }
}

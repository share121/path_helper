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

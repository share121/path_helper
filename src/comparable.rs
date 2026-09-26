use std::path::{Path, PathBuf};

/// Normalize a path into a form that compares consistently across Windows path
/// representations.
///
/// On Windows the same location can be spelled with or without the
/// extended-length prefix: [`soft_canonicalize`] always returns verbatim paths
/// (`\\?\C:\…`), while `dunce` strips the `\\?\` prefix *only when safe* — so
/// two paths that refer to the same location can come back as `C:\dl` and
/// `\\?\C:\dl\sub` depending on length, reserved names, and so on.
/// [`Path::starts_with`] compares component-wise, and a `Disk` prefix is not
/// equal to a `VerbatimDisk` prefix, so such a pair would not match.
///
/// This mirrors [`dunce::simplified`] (verbatim disk → drive, verbatim UNC →
/// UNC) so the results can be compared with `starts_with` / `==`. It is a
/// **comparison helper only**: no I/O is performed and no new path is created,
/// so keep using the original path for filesystem access.
///
/// # Not covered
///
/// This is a spelling normalization, not a filesystem resolution. It does
/// **not**:
///
/// - fold case (Windows paths are case-insensitive, this is a literal compare);
/// - resolve symlinks or junctions;
/// - expand 8.3 short names (`PROGRA~1` stays distinct from `Program Files`).
///
/// Use a canonicalization function (e.g. [`soft_canonicalize`]) when those
/// matter, and apply this to its output only to make the two spellings
/// comparable.
///
/// [`soft_canonicalize`]: https://docs.rs/soft-canonicalize
/// [`dunce::simplified`]: https://docs.rs/dunce
#[must_use]
#[cfg(windows)]
pub fn comparable(path: impl AsRef<Path>) -> PathBuf {
    use std::path::{Component, Prefix};

    let path = path.as_ref();
    let mut comps = path.components();
    let Some(Component::Prefix(prefix)) = comps.next() else {
        return path.to_path_buf();
    };
    let mut out = match prefix.kind() {
        Prefix::VerbatimDisk(drive) => PathBuf::from(format!("{}:\\", drive as char)),
        Prefix::VerbatimUNC(server, share) => {
            let mut p = PathBuf::from(r"\\");
            p.push(server);
            p.push(share);
            p
        }
        // `Verbatim` (volume GUIDs), `DeviceNS`, plain `Disk`/`UNC`: nothing to
        // simplify, keep as-is.
        _ => return path.to_path_buf(),
    };
    for c in comps {
        if !matches!(c, Component::RootDir) {
            out.push(c.as_os_str());
        }
    }
    out
}

/// Normalize a path into a form that compares consistently across Windows path
/// representations.
///
/// On non-Windows platforms there is only one representation, so this is the
/// identity function. See the Windows implementation for the full contract.
#[must_use]
#[cfg(not(windows))]
pub fn comparable(path: impl AsRef<Path>) -> PathBuf {
    path.as_ref().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[cfg(windows)]
    #[test]
    fn strips_verbatim_disk_prefix() {
        assert_eq!(
            comparable(Path::new(r"\\?\C:\dl\a\b")),
            PathBuf::from(r"C:\dl\a\b")
        );
    }

    #[cfg(windows)]
    #[test]
    fn plain_path_is_unchanged() {
        assert_eq!(comparable(Path::new(r"C:\dl")), PathBuf::from(r"C:\dl"));
    }

    #[cfg(windows)]
    #[test]
    fn mixed_representations_compare_as_contained() {
        // A short `save_dir` can be simplified to `C:\dl` while a longer child
        // stays verbatim — raw `starts_with` then fails, but the normalized
        // comparison succeeds.
        let base = Path::new(r"C:\dl");
        let child = Path::new(r"\\?\C:\dl\sub\deeper");
        assert!(!child.starts_with(base));
        assert!(comparable(child).starts_with(comparable(base)));
    }

    #[cfg(windows)]
    #[test]
    fn genuine_escape_is_not_contained() {
        let base = Path::new(r"C:\dl");
        let escape = Path::new(r"\\?\C:\other\x");
        assert!(!comparable(escape).starts_with(comparable(base)));
    }

    #[cfg(windows)]
    #[test]
    fn sibling_with_common_string_prefix_is_not_contained() {
        // `DownloadsExtra` shares the textual prefix `Downloads`, but paths are
        // compared component-wise, so it must not count as contained.
        let base = Path::new(r"\\?\C:\dl\Downloads");
        let sibling = Path::new(r"C:\dl\DownloadsExtra");
        assert!(!comparable(sibling).starts_with(comparable(base)));
    }

    #[cfg(windows)]
    #[test]
    fn drive_root_contains_subtree() {
        assert!(comparable(Path::new(r"\\?\C:\dl")).starts_with(comparable(Path::new(r"C:\"))));
    }

    #[cfg(windows)]
    #[test]
    fn same_path_is_contained_in_itself() {
        assert!(comparable(Path::new(r"\\?\C:\dl")).starts_with(comparable(Path::new(r"C:\dl"))));
    }

    #[cfg(windows)]
    #[test]
    fn strips_verbatim_unc_prefix() {
        assert_eq!(
            comparable(Path::new(r"\\?\UNC\srv\share\a")),
            PathBuf::from(r"\\srv\share\a")
        );
        assert!(comparable(Path::new(r"\\?\UNC\srv\share\a"))
            .starts_with(comparable(Path::new(r"\\srv\share"))));
    }

    #[cfg(windows)]
    #[test]
    fn verbatim_volume_guid_is_left_alone() {
        let p = Path::new(r"\\?\Volume{00000000-0000-0000-0000-000000000000}\a");
        assert_eq!(comparable(p), p.to_path_buf());
    }

    #[cfg(windows)]
    #[test]
    fn relative_path_is_left_alone() {
        assert_eq!(comparable(Path::new(r"a\b")), PathBuf::from(r"a\b"));
    }

    #[cfg(not(windows))]
    #[test]
    fn identity_on_unix() {
        assert_eq!(comparable(Path::new("/dl/a")), PathBuf::from("/dl/a"));
    }
}

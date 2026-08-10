use path_helper::{add_file_stem_prefix, gen_unique_path, is_extension, safe_replace, IterStemExt};
use std::fs;
use std::path::{Path, PathBuf};

#[must_use]
fn tmp_root(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ph_sync_{}_{}", std::process::id(), name));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn safe_replace_then_read() {
    let dir = tmp_root("sr");
    let p = dir.join("f.txt");
    safe_replace(&p, b"data").unwrap();
    assert_eq!(fs::read(&p).unwrap(), b"data");
    fs::remove_file(&p).unwrap();
}

#[test]
fn gen_unique_path_finds_free() {
    let dir = tmp_root("gu");
    let base = dir.join("x.txt");
    fs::write(&base, b"0").unwrap();
    let free = gen_unique_path(&base).unwrap();
    assert_eq!(free, dir.join("x (1).txt"));
    safe_replace(&free, b"1").unwrap();
    assert_eq!(fs::read(&free).unwrap(), b"1");
    fs::remove_file(&base).unwrap();
    fs::remove_file(&free).unwrap();
}

#[test]
fn iter_stem_feeds_unique_path() {
    let dir = tmp_root("is");
    let base = dir.join("dup.txt");
    fs::write(&base, b"orig").unwrap();
    let free = base.iter_stem().take(10).find(|c| !c.exists()).unwrap();
    assert_ne!(free, base);
    // 真实地把这个空闲名用掉，验证 iter_stem 确实产出可用路径
    safe_replace(&free, b"new").unwrap();
    assert!(free.exists());
    fs::remove_file(&base).unwrap();
    fs::remove_file(&free).unwrap();
}

#[test]
fn file_stem_prefix_integration() {
    let p = Path::new("a/b/c.txt");
    assert_eq!(
        add_file_stem_prefix(p, "pre"),
        PathBuf::from("a/b/prec.txt")
    );
    assert!(is_extension("txt"));
}

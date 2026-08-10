#![cfg(feature = "sanitize")]
use path_helper::{sanitize_filename, sanitize_path};
use std::path::Path;

#[test]
fn sanitize_filename_keeps_ext() {
    assert_eq!(sanitize_filename("my file*.txt", 255), "my file_.txt");
    let long = format!("{}.mp4", "x".repeat(300));
    let r = sanitize_filename(&long, 255);
    assert!(Path::new(&r)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("mp4")));
}

#[test]
fn sanitize_path_roundtrip_simple() {
    let p = Path::new("a/b/c.txt");
    let r = sanitize_path(p);
    assert_eq!(r.to_string_lossy().replace('\\', "/"), "a/b/c.txt");
}

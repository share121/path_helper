#![cfg(feature = "tokio")]
use path_helper::tokio::{gen_unique_path, safe_replace};
use std::fs;

#[tokio::test]
async fn safe_replace_async() {
    let dir = std::env::temp_dir().join(format!("ph_tok_{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let p = dir.join("a.txt");
    safe_replace(&p, b"hi").await.unwrap();
    assert_eq!(fs::read(&p).unwrap(), b"hi");
    fs::remove_file(&p).unwrap();
}

#[tokio::test]
async fn gen_unique_path_async() {
    let dir = std::env::temp_dir().join(format!("ph_tok2_{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let target = dir.join("b.txt");
    fs::write(&target, b"0").unwrap();
    let got = gen_unique_path(&target).await.unwrap();
    assert_eq!(got, dir.join("b (1).txt"));
    fs::remove_file(&target).unwrap();
    fs::remove_file(&got).unwrap();
}

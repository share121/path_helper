#![cfg(feature = "auto_ext")]
use path_helper::auto_ext;
use std::borrow::Cow;

#[test]
fn auto_ext_adds_from_content_type() {
    assert_eq!("img.png", auto_ext("img", Some("image/png")));
}

#[test]
fn auto_ext_returns_borrowed_when_valid() {
    let s = String::from("doc.pdf");
    let r = auto_ext(&s, Some("application/pdf"));
    assert!(matches!(r, Cow::Borrowed(_)));
    assert_eq!("doc.pdf", r);
}

#[test]
fn auto_ext_owns_when_appending() {
    let r = auto_ext("img", Some("image/png"));
    assert!(matches!(r, Cow::Owned(_)));
}

// SPDX-License-Identifier: MIT

use super::sha256_file;
use std::sync::atomic::AtomicBool;

#[test]
fn hashes_file_contents_across_chunks() {
    let root = tempfile::tempdir().expect("fixture");
    let cases: [(&[u8], &str); 2] = [
        (
            b"",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
    ];
    for (contents, expected) in cases {
        let path = root.path().join("file");
        std::fs::write(&path, contents).expect("write");
        let hash = sha256_file(&path, &AtomicBool::new(false)).expect("hash");
        assert_eq!(hash.as_deref(), Some(expected));
    }
    let large = root.path().join("large");
    std::fs::write(&large, vec![b'a'; 1_000_000 * 3]).expect("write");
    let in_one = ring::digest::digest(&ring::digest::SHA256, &vec![b'a'; 1_000_000 * 3]);
    let expected: String = in_one.as_ref().iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        sha256_file(&large, &AtomicBool::new(false)).expect("hash"),
        Some(expected)
    );
}

#[test]
fn stops_when_cancelled_and_reports_missing_files() {
    let root = tempfile::tempdir().expect("fixture");
    let path = root.path().join("file");
    std::fs::write(&path, b"contents").expect("write");
    assert_eq!(
        sha256_file(&path, &AtomicBool::new(true)).expect("hash"),
        None
    );
    assert!(sha256_file(&root.path().join("missing"), &AtomicBool::new(false)).is_err());
}

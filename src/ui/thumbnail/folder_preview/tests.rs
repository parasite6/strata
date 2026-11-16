// SPDX-License-Identifier: MIT

use super::preview_children;
use std::path::{Path, PathBuf};

fn write(folder: &Path, name: &str, bytes: &[u8]) {
    std::fs::write(folder.join(name), bytes).unwrap();
}

fn names(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn chooses_pictures_and_videos_by_name() {
    let folder = tempfile::tempdir().unwrap();
    for name in ["c.png", "a.jpg", "b.mp4", "d.webp", "e.png"] {
        write(folder.path(), name, b"x");
    }
    assert_eq!(
        names(&preview_children(folder.path(), false)),
        ["a.jpg", "b.mp4", "c.png", "d.webp"]
    );
}

#[test]
fn audio_and_other_files_are_not_chosen() {
    let folder = tempfile::tempdir().unwrap();
    for name in ["song.mp3", "track.flac", "notes.txt", "doc.pdf", "model.stl", "x.png"] {
        write(folder.path(), name, b"x");
    }
    assert_eq!(names(&preview_children(folder.path(), false)), ["x.png"]);
}

#[test]
fn skips_symlinks_empty_files_and_folders() {
    let folder = tempfile::tempdir().unwrap();
    write(folder.path(), "real.png", b"x");
    write(folder.path(), "empty.png", b"");
    std::os::unix::fs::symlink(folder.path().join("real.png"), folder.path().join("link.png"))
        .unwrap();
    std::fs::create_dir(folder.path().join("sub.png")).unwrap();
    assert_eq!(names(&preview_children(folder.path(), false)), ["real.png"]);
}

#[test]
fn hidden_pictures_follow_show_hidden() {
    let folder = tempfile::tempdir().unwrap();
    write(folder.path(), ".secret.png", b"x");
    write(folder.path(), "shown.png", b"x");
    assert_eq!(names(&preview_children(folder.path(), false)), ["shown.png"]);
    assert_eq!(
        names(&preview_children(folder.path(), true)),
        [".secret.png", "shown.png"]
    );
}

#[test]
fn unreadable_folder_yields_nothing() {
    let folder = tempfile::tempdir().unwrap();
    assert!(preview_children(&folder.path().join("missing"), false).is_empty());
}

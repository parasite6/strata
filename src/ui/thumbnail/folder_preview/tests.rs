// SPDX-License-Identifier: MIT

use super::preview_children;
use std::path::{Path, PathBuf};

fn write(folder: &Path, name: &str, bytes: &[u8]) {
    std::fs::write(folder.join(name), bytes).expect("test fixture");
}

fn names(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|path| {
            path.file_name()
                .expect("test fixture")
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

#[test]
fn chooses_pictures_and_videos_by_name() {
    let folder = tempfile::tempdir().expect("test fixture");
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
    let folder = tempfile::tempdir().expect("test fixture");
    for name in [
        "song.mp3",
        "track.flac",
        "notes.txt",
        "doc.pdf",
        "model.stl",
        "x.png",
    ] {
        write(folder.path(), name, b"x");
    }
    assert_eq!(names(&preview_children(folder.path(), false)), ["x.png"]);
}

#[test]
fn skips_symlinks_empty_files_and_folders() {
    let folder = tempfile::tempdir().expect("test fixture");
    write(folder.path(), "real.png", b"x");
    write(folder.path(), "empty.png", b"");
    std::os::unix::fs::symlink(
        folder.path().join("real.png"),
        folder.path().join("link.png"),
    )
    .expect("test fixture");
    std::fs::create_dir(folder.path().join("sub.png")).expect("test fixture");
    assert_eq!(names(&preview_children(folder.path(), false)), ["real.png"]);
}

#[test]
fn hidden_pictures_follow_show_hidden() {
    let folder = tempfile::tempdir().expect("test fixture");
    write(folder.path(), ".secret.png", b"x");
    write(folder.path(), "shown.png", b"x");
    assert_eq!(
        names(&preview_children(folder.path(), false)),
        ["shown.png"]
    );
    assert_eq!(
        names(&preview_children(folder.path(), true)),
        [".secret.png", "shown.png"]
    );
}

#[test]
fn unreadable_folder_yields_nothing() {
    let folder = tempfile::tempdir().expect("test fixture");
    assert!(preview_children(&folder.path().join("missing"), false).is_empty());
}

fn folder_entry(path: &Path) -> crate::model::FileEntry {
    use crate::model::{EntryKind, FileEntry, Location, MetadataValue};
    FileEntry {
        location: Location::local(path),
        native_name: path.file_name().unwrap_or_default().to_owned(),
        thumbnail_path: None,
        display_name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        kind: if path.is_dir() {
            EntryKind::Directory
        } else {
            EntryKind::File
        },
        recent_unix_seconds: MetadataValue::Unknown,
        is_hidden: false,
        mode: MetadataValue::Unknown,
        size: MetadataValue::Unknown,
        modified_unix_seconds: MetadataValue::Unknown,
        created_unix_seconds: MetadataValue::Unknown,
        image_dimensions: MetadataValue::Unknown,
        child_count: MetadataValue::Unknown,
        duration_seconds: MetadataValue::Unknown,
        listable: MetadataValue::Unknown,
        recent_uri: None,
    }
}

fn settle() {
    let context = gtk::glib::MainContext::default();
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
    while std::time::Instant::now() < deadline {
        while context.iteration(false) {}
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

const SETTLED: super::FolderBinding = super::FolderBinding {
    show_hidden: false,
    thumbnail_size: 96,
    scrolling: false,
};

fn bound(slot: &super::super::ThumbnailSlot, path: &Path) -> usize {
    super::bind_folder_previews(slot, &folder_entry(path), SETTLED);
    settle();
    slot.folder_preview_children().len()
}

fn picture_folder(root: &Path) -> PathBuf {
    let folder = root.join("pictures");
    std::fs::create_dir(&folder).expect("test fixture");
    write(&folder, "a.png", b"x");
    write(&folder, "b.png", b"x");
    folder
}

fn plain_folder(root: &Path) -> PathBuf {
    let folder = root.join("plain");
    std::fs::create_dir(&folder).expect("test fixture");
    write(&folder, "notes.txt", b"x");
    folder
}

#[test]
fn picture_folder_gets_child_previews() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::picture_folder_gets_child_previews",
        || {
            let root = tempfile::tempdir().expect("test fixture");
            let slot = super::super::ThumbnailSlot::new(96);
            assert_eq!(bound(&slot, &picture_folder(root.path())), 2);
        },
    );
}

#[test]
fn customizing_a_shown_folder_drops_and_restores_previews() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::customizing_a_shown_folder_drops_and_restores_previews",
        || {
            let manager = crate::ui::preferences::PreferenceManager::shared();
            let root = tempfile::tempdir().expect("test fixture");
            let folder = picture_folder(root.path());
            let slot = super::super::ThumbnailSlot::new(96);
            let icon = crate::assets::icons::FOLDER;
            super::super::set_thumbnail_or_icon(&slot, &folder_entry(&folder), icon, 96, 96);
            assert_eq!(bound(&slot, &folder), 2);
            manager.set_custom_icon(
                &folder,
                Some(crate::assets::icons::CUSTOMIZATION_CHOICES[0].0),
            );
            settle();
            assert!(slot.folder_preview_children().is_empty());
            manager.set_custom_icon(&folder, None);
            settle();
            assert_eq!(slot.folder_preview_children().len(), 2);
        },
    );
}

#[test]
fn scrolling_defers_the_scan_until_resumed() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::scrolling_defers_the_scan_until_resumed",
        || {
            let root = tempfile::tempdir().expect("test fixture");
            let folder = picture_folder(root.path());
            let slot = super::super::ThumbnailSlot::new(96);
            let scrolling = super::FolderBinding {
                scrolling: true,
                ..SETTLED
            };
            super::bind_folder_previews(&slot, &folder_entry(&folder), scrolling);
            settle();
            assert!(slot.folder_preview_children().is_empty());
            super::resume_folder_previews(&slot);
            settle();
            assert_eq!(slot.folder_preview_children().len(), 2);
        },
    );
}

#[test]
fn plain_folder_gets_none() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::plain_folder_gets_none",
        || {
            let root = tempfile::tempdir().expect("test fixture");
            let slot = super::super::ThumbnailSlot::new(96);
            assert_eq!(bound(&slot, &plain_folder(root.path())), 0);
        },
    );
}

#[test]
fn custom_icon_folder_gets_none() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::custom_icon_folder_gets_none",
        || {
            let root = tempfile::tempdir().expect("test fixture");
            let folder = picture_folder(root.path());
            crate::ui::preferences::PreferenceManager::shared().set_custom_icon(
                &folder,
                Some(crate::assets::icons::CUSTOMIZATION_CHOICES[0].0),
            );
            let slot = super::super::ThumbnailSlot::new(96);
            assert_eq!(bound(&slot, &folder), 0);
        },
    );
}

#[test]
fn rebinding_to_a_plain_folder_clears_previews() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::rebinding_to_a_plain_folder_clears_previews",
        || {
            let root = tempfile::tempdir().expect("test fixture");
            let slot = super::super::ThumbnailSlot::new(96);
            assert_eq!(bound(&slot, &picture_folder(root.path())), 2);
            assert_eq!(bound(&slot, &plain_folder(root.path())), 0);
        },
    );
}

#[test]
fn a_stale_scan_does_not_attach() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::a_stale_scan_does_not_attach",
        || {
            let root = tempfile::tempdir().expect("test fixture");
            let folder = picture_folder(root.path());
            let slot = super::super::ThumbnailSlot::new(96);
            super::bind_folder_previews(&slot, &folder_entry(&folder), SETTLED);
            assert_eq!(bound(&slot, &folder.join("a.png")), 0);
        },
    );
}

#[test]
fn turning_thumbnails_off_clears_previews() {
    crate::test_support::gtk_test(
        "ui::thumbnail::folder_preview::tests::turning_thumbnails_off_clears_previews",
        || {
            let manager = crate::ui::preferences::PreferenceManager::shared();
            let root = tempfile::tempdir().expect("test fixture");
            let slot = super::super::ThumbnailSlot::new(96);
            assert_eq!(bound(&slot, &picture_folder(root.path())), 2);
            manager.set_show_thumbnails(false);
            settle();
            assert!(slot.folder_preview_children().is_empty());
            manager.set_show_thumbnails(true);
            settle();
            assert_eq!(slot.folder_preview_children().len(), 2);
        },
    );
}

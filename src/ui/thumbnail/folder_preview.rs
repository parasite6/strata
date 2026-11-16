// SPDX-License-Identifier: MIT

//! Dolphin-style folder icons: up to four of a folder's pictures and videos drawn on it.

use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
};

use gtk::{gio, glib, prelude::*};

use super::{ThumbnailSlot, kind::is_folder_preview_kind, known_metadata};
use crate::model::FileEntry;

mod draw;
pub(super) use draw::{draw_folder_previews, tile_rects};

pub(super) const MAX_PREVIEW_TILES: usize = 4;
const MAX_SCANNED_ENTRIES: usize = 500;
const MAX_REMEMBERED_FOLDERS: usize = 512;

/// The folder a slot previews, kept so preference changes can redo the binding.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct FolderSource {
    path: PathBuf,
    modified: Option<i64>,
    show_hidden: bool,
    thumbnail_size: i32,
}

thread_local! {
    static CHOSEN: RefCell<HashMap<FolderSource, Vec<PathBuf>>> = RefCell::new(HashMap::new());
}

/// How a folder icon binds its previews.
#[derive(Clone, Copy)]
pub(in crate::ui) struct FolderBinding {
    pub(in crate::ui) show_hidden: bool,
    pub(in crate::ui) thumbnail_size: i32,
    /// Scans wait for [`resume_folder_previews`] while the view scrolls.
    pub(in crate::ui) scrolling: bool,
}

/// Shows up to four of a local folder's pictures and videos on its icon; anything else clears them.
pub(in crate::ui) fn bind_folder_previews(
    slot: &ThumbnailSlot,
    entry: &FileEntry,
    binding: FolderBinding,
) {
    let source = folder_source(entry, binding.show_hidden, binding.thumbnail_size);
    let previous = slot.replace_folder_source(source.clone());
    if previous.as_ref().map(|old| &old.path) != source.as_ref().map(|new| &new.path) {
        slot.clear_folder_previews();
    }
    apply(slot, source, !binding.scrolling);
}

/// Starts the scan a binding deferred while the view scrolled.
pub(in crate::ui) fn resume_folder_previews(slot: &ThumbnailSlot) {
    if slot.replace_folder_scan_deferred(false) {
        apply(slot, slot.folder_source(), true);
    }
}

pub(super) fn refresh(slot: &ThumbnailSlot) {
    apply(slot, slot.folder_source(), true);
}

fn folder_source(
    entry: &FileEntry,
    show_hidden: bool,
    thumbnail_size: i32,
) -> Option<FolderSource> {
    if !entry.is_directory() || entry.location.is_administrator() {
        return None;
    }
    let path = entry.location.native_path()?;
    if crate::adapters::mount_watch::latest().is_remote_share(path) {
        return None;
    }
    Some(FolderSource {
        path: path.to_path_buf(),
        modified: known_metadata(&entry.modified_unix_seconds),
        show_hidden,
        thumbnail_size,
    })
}

fn previews_allowed(folder: &Path) -> bool {
    let preferences = crate::ui::preferences::PreferenceManager::shared();
    preferences.show_thumbnails()
        && preferences.folder_previews()
        && preferences.custom_icon(folder).is_none()
}

fn apply(slot: &ThumbnailSlot, source: Option<FolderSource>, scan: bool) {
    let generation = slot.next_folder_generation();
    slot.replace_folder_scan_deferred(false);
    let Some(source) = source.filter(|source| previews_allowed(&source.path)) else {
        slot.clear_folder_previews();
        return;
    };
    if let Some(chosen) = CHOSEN.with_borrow(|chosen| chosen.get(&source).cloned()) {
        attach(slot, &source, &chosen);
        return;
    }
    if !scan {
        slot.replace_folder_scan_deferred(true);
        return;
    }
    let slot = slot.downgrade();
    glib::spawn_future_local(async move {
        let (folder, show_hidden) = (source.path.clone(), source.show_hidden);
        let chosen = gio::spawn_blocking(move || preview_children(&folder, show_hidden))
            .await
            .unwrap_or_default();
        CHOSEN.with_borrow_mut(|remembered| {
            if remembered.len() >= MAX_REMEMBERED_FOLDERS {
                remembered.clear();
            }
            remembered.insert(source.clone(), chosen.clone());
        });
        if let Some(slot) = slot
            .upgrade()
            .filter(|slot| slot.folder_generation() == generation)
        {
            attach(&slot, &source, &chosen);
        }
    });
}

fn attach(slot: &ThumbnailSlot, source: &FolderSource, chosen: &[PathBuf]) {
    let shown: Vec<Option<PathBuf>> = slot
        .folder_preview_children()
        .iter()
        .map(|child| child.thumbnail_source().map(|source| source.path))
        .collect();
    if shown.len() == chosen.len()
        && shown
            .iter()
            .zip(chosen)
            .all(|(old, new)| old.as_ref() == Some(new))
    {
        return;
    }
    let size = (source.thumbnail_size / 2).max(16);
    let children: Vec<ThumbnailSlot> = chosen
        .iter()
        .map(|path| {
            let child = ThumbnailSlot::new(size);
            // Children are never drawn themselves, so any fallback icon serves.
            super::set_thumbnail_or_icon_for_path(
                &child,
                path,
                crate::assets::icons::PICTURES,
                size,
                size,
            );
            child
        })
        .collect();
    slot.set_folder_previews(&children);
}

/// The folder's first top-level pictures and videos by name, skipping links and empty files.
pub(super) fn preview_children(folder: &Path, show_hidden: bool) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut chosen: Vec<PathBuf> = entries
        .take(MAX_SCANNED_ENTRIES)
        .filter_map(Result::ok)
        .filter(|entry| show_hidden || !entry.file_name().as_encoded_bytes().starts_with(b"."))
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|entry| entry.path())
        .filter(|path| is_folder_preview_kind(path))
        .filter(|path| std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.len() > 0))
        .collect();
    chosen.sort_by(|left, right| left.file_name().cmp(&right.file_name()));
    chosen.truncate(MAX_PREVIEW_TILES);
    chosen
}

#[cfg(test)]
mod tests;

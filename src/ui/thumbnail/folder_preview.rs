// SPDX-License-Identifier: MIT

//! Dolphin-style folder icons: up to four of a folder's pictures and videos drawn on it.

use std::path::{Path, PathBuf};

use super::kind::is_folder_preview_kind;

pub(super) const MAX_PREVIEW_TILES: usize = 4;
const MAX_SCANNED_ENTRIES: usize = 500;

/// The folder's first top-level pictures and videos by name, skipping links and empty files.
pub(super) fn preview_children(folder: &Path, show_hidden: bool) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut chosen: Vec<PathBuf> = entries
        .take(MAX_SCANNED_ENTRIES)
        .filter_map(Result::ok)
        .filter(|entry| show_hidden || !entry.file_name().as_encoded_bytes().starts_with(b"."))
        .filter(|entry| {
            entry
                .metadata()
                .is_ok_and(|metadata| metadata.file_type().is_file() && metadata.len() > 0)
        })
        .map(|entry| entry.path())
        .filter(|path| is_folder_preview_kind(path))
        .collect();
    chosen.sort_by(|left, right| left.file_name().cmp(&right.file_name()));
    chosen.truncate(MAX_PREVIEW_TILES);
    chosen
}

#[cfg(test)]
mod tests;

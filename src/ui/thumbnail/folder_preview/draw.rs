// SPDX-License-Identifier: MIT

//! Draws a folder's child thumbnails over its icon: tilted, framed and shadowed like Dolphin.

use std::{
    cell::RefCell,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use gtk::{gdk, glib, graphene, gsk, prelude::*};

use super::super::ThumbnailSlot;

const MAX_TILT_DEGREES: u64 = 8;
const SHADOW_ALPHA: f32 = 0.5;
const MIN_TILE: f32 = 5.0;

/// Dolphin's tile grid inside the folder body: one tile fills it, more share a 2×2 grid.
pub(in crate::ui::thumbnail) fn tile_rects(body: graphene::Rect, count: usize) -> Vec<graphene::Rect> {
    let (width, height) = (body.width(), body.height());
    let (top, bottom, side, gap) = (height * 0.3, height / 6.0, width / 13.0, 1.0);
    let area = graphene::Rect::new(
        body.x() + side,
        body.y() + top,
        width - 2.0 * side,
        height - top - bottom,
    );
    if count == 1 {
        return vec![area];
    }
    let (tile_width, tile_height) = ((area.width() - gap) / 2.0, (area.height() - gap) / 2.0);
    (0..count.min(4))
        .map(|index| {
            let (column, row) = ((index % 2) as f32, (index / 2) as f32);
            graphene::Rect::new(
                area.x() + column * (tile_width + gap),
                area.y() + row * (tile_height + gap),
                tile_width,
                tile_height,
            )
        })
        .collect()
}

pub(in crate::ui::thumbnail) fn draw_folder_previews(
    snapshot: &gtk::Snapshot,
    slot: &ThumbnailSlot,
    body: graphene::Rect,
) {
    let shown: Vec<(gdk::Texture, Option<std::path::PathBuf>)> = slot
        .folder_preview_children()
        .iter()
        .filter_map(|child| {
            let path = child.thumbnail_source().map(|source| source.path);
            child.texture().map(|texture| (texture, path))
        })
        .collect();
    let tiles = tile_rects(body, shown.len());
    let stroke = (body.width() / 170.0).round().max(1.0);
    let frame = slot.color();
    for ((texture, path), tile) in shown.iter().zip(tiles) {
        if tile.width() < MIN_TILE || tile.height() < MIN_TILE {
            return;
        }
        draw_tile(snapshot, texture, tile, TileStyle { stroke, frame, tilt: tilt(path.as_deref()) });
    }
}

struct TileStyle {
    stroke: f32,
    frame: gdk::RGBA,
    tilt: f32,
}

fn draw_tile(snapshot: &gtk::Snapshot, texture: &gdk::Texture, tile: graphene::Rect, style: TileStyle) {
    let framed = is_opaque(texture);
    let border = if framed { style.stroke } else { 0.0 };
    let (image_width, image_height) = fit(texture, tile.width() - 2.0 * border, tile.height() - 2.0 * border);
    let (outer_width, outer_height) = (image_width + 2.0 * border, image_height + 2.0 * border);
    snapshot.save();
    snapshot.translate(&graphene::Point::new(
        tile.x() + tile.width() / 2.0,
        tile.y() + tile.height() / 2.0,
    ));
    snapshot.rotate(style.tilt);
    let outer = graphene::Rect::new(-outer_width / 2.0, -outer_height / 2.0, outer_width, outer_height);
    let corner = graphene::Size::new(border / 2.0, border / 2.0);
    let rounded = gsk::RoundedRect::new(outer, corner, corner, corner, corner);
    snapshot.append_outset_shadow(
        &rounded,
        &gdk::RGBA::new(0.0, 0.0, 0.0, SHADOW_ALPHA),
        0.0,
        0.0,
        0.0,
        style.stroke,
    );
    if framed {
        snapshot.push_rounded_clip(&rounded);
        snapshot.append_color(&style.frame, &outer);
        snapshot.pop();
    }
    snapshot.append_texture(
        texture,
        &graphene::Rect::new(-image_width / 2.0, -image_height / 2.0, image_width, image_height),
    );
    snapshot.restore();
}

fn fit(texture: &gdk::Texture, width: f32, height: f32) -> (f32, f32) {
    let (natural_width, natural_height) = (texture.width().max(1) as f32, texture.height().max(1) as f32);
    let scale = (width / natural_width).min(height / natural_height).max(0.0);
    (natural_width * scale, natural_height * scale)
}

/// A stable tilt of −8°…+8° per file, so a folder looks the same on every visit.
fn tilt(path: Option<&std::path::Path>) -> f32 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    (hasher.finish() % (2 * MAX_TILT_DEGREES + 1)) as f32 - MAX_TILT_DEGREES as f32
}

thread_local! {
    static OPAQUE: RefCell<HashMap<usize, (glib::WeakRef<gdk::Texture>, bool)>> = RefCell::new(HashMap::new());
}

/// Transparent artwork such as logos gets no frame; photos do.
fn is_opaque(texture: &gdk::Texture) -> bool {
    let key = texture.as_ptr() as usize;
    OPAQUE.with_borrow_mut(|known| {
        if let Some((owner, opaque)) = known.get(&key)
            && owner.upgrade().as_ref() == Some(texture)
        {
            return *opaque;
        }
        if known.len() >= 512 {
            known.retain(|_, (owner, _)| owner.upgrade().is_some());
        }
        let mut downloader = gdk::TextureDownloader::new(texture);
        downloader.set_format(gdk::MemoryFormat::R8g8b8a8);
        let (pixels, stride) = downloader.download_bytes();
        let width = texture.width() as usize;
        let opaque = (0..texture.height() as usize)
            .all(|row| (0..width).all(|column| pixels[row * stride + column * 4 + 3] == u8::MAX));
        known.insert(key, (texture.downgrade(), opaque));
        opaque
    })
}

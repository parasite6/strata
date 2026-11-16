// SPDX-License-Identifier: MIT

//! File and folder icons drawn from an installed freedesktop icon theme.
//!
//! Callers get an interned token `builtin\x1fname;name…` instead of a bundled
//! icon name. Painting resolves the names against the selected theme and falls
//! back to the bundled icon, so switching themes only needs a repaint.

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use gtk::{gdk, gio, glib, prelude::*};
use resvg::{tiny_skia, usvg};

pub const BUILT_IN: &str = "strata";
pub const FOLLOW_SYSTEM: &str = "system";

const SEPARATOR: char = '\u{1f}';
const NAME_CACHE_LIMIT: usize = 4096;
const MAX_IMAGE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_IMAGE_DIMENSION: u32 = 4096;

thread_local! {
    static SELECTED: RefCell<String> = RefCell::new(BUILT_IN.to_owned());
    static ACTIVE: RefCell<Option<ActiveTheme>> = const { RefCell::new(None) };
    static TOKENS: RefCell<HashSet<&'static str>> = RefCell::new(HashSet::new());
    static FILE_TOKENS: RefCell<HashMap<(&'static str, String), &'static str>> =
        RefCell::new(HashMap::new());
}

struct ActiveTheme {
    theme: gtk::IconTheme,
    resolved: HashMap<String, Option<PathBuf>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstalledTheme {
    pub id: String,
    pub name: String,
}

/// Returns whether the effective theme changed.
pub fn select(choice: &str) -> bool {
    let unchanged = SELECTED.with_borrow(|selected| selected == choice);
    if unchanged && choice != FOLLOW_SYSTEM {
        return false;
    }
    SELECTED.replace(choice.to_owned());
    let name = theme_name_for(choice);
    let current = ACTIVE.with_borrow(|active| {
        active
            .as_ref()
            .map(|active| active.theme.theme_name().to_string())
    });
    if current == name {
        return false;
    }
    ACTIVE.replace(name.map(|name| {
        let theme = gtk::IconTheme::new();
        theme.set_theme_name(Some(&name));
        ActiveTheme {
            theme,
            resolved: HashMap::new(),
        }
    }));
    true
}

fn theme_name_for(choice: &str) -> Option<String> {
    match choice {
        BUILT_IN | "" => None,
        FOLLOW_SYSTEM => gtk::Settings::default()
            .and_then(|settings| settings.gtk_icon_theme_name())
            .map(|name| name.to_string())
            .filter(|name| !name.is_empty()),
        theme => Some(theme.to_owned()),
    }
}

/// The bundled icon a token falls back to; plain bundled names pass through.
pub fn builtin(name: &str) -> &str {
    name.split_once(SEPARATOR)
        .map_or(name, |(builtin, _)| builtin)
}

fn token(builtin: &'static str, names: &[&str]) -> &'static str {
    let key = format!("{builtin}{SEPARATOR}{}", names.join(";"));
    TOKENS.with_borrow_mut(|tokens| {
        if let Some(existing) = tokens.get(key.as_str()) {
            return *existing;
        }
        let interned: &'static str = Box::leak(key.into_boxed_str());
        tokens.insert(interned);
        interned
    })
}

/// A theme icon for an item's content type, guessed from its name alone.
pub fn file_icon(builtin: &'static str, file_name: &str) -> &'static str {
    let key = (builtin, file_name.to_lowercase());
    if let Some(found) = FILE_TOKENS.with_borrow(|cache| cache.get(&key).copied()) {
        return found;
    }
    let names = content_type_icon_names(file_name);
    let names = names.iter().map(String::as_str).collect::<Vec<_>>();
    let found = token(builtin, &names);
    FILE_TOKENS.with_borrow_mut(|cache| {
        if cache.len() >= NAME_CACHE_LIMIT {
            cache.clear();
        }
        cache.insert(key, found);
    });
    found
}

fn content_type_icon_names(file_name: &str) -> Vec<String> {
    let (content_type, _) = gio::content_type_guess(Some(file_name), None::<&[u8]>);
    let mut names = gio::content_type_get_icon(&content_type)
        .downcast::<gio::ThemedIcon>()
        .map(|icon| {
            icon.names()
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    names.retain(|name| !name.ends_with("-symbolic"));
    names
}

pub fn folder_icon(builtin: &'static str, path: Option<&Path>) -> &'static str {
    match path.and_then(special_folder_name) {
        Some(special) => token(builtin, &[special, "folder"]),
        None => token(builtin, &["folder"]),
    }
}

pub fn named_icon(builtin: &'static str, names: &[&str]) -> &'static str {
    token(builtin, names)
}

fn special_folder_name(path: &Path) -> Option<&'static str> {
    if path == glib::home_dir() {
        return Some("user-home");
    }
    [
        (glib::UserDirectory::Desktop, "user-desktop"),
        (glib::UserDirectory::Documents, "folder-documents"),
        (glib::UserDirectory::Downloads, "folder-download"),
        (glib::UserDirectory::Music, "folder-music"),
        (glib::UserDirectory::Pictures, "folder-pictures"),
        (glib::UserDirectory::PublicShare, "folder-publicshare"),
        (glib::UserDirectory::Templates, "folder-templates"),
        (glib::UserDirectory::Videos, "folder-videos"),
    ]
    .into_iter()
    .find(|(directory, _)| glib::user_special_dir(*directory).as_deref() == Some(path))
    .map(|(_, name)| name)
}

/// The theme file a token resolves to, or `None` to draw the bundled icon.
pub(super) fn themed_file(name: &str) -> Option<PathBuf> {
    let (_, request) = name.split_once(SEPARATOR)?;
    ACTIVE.with_borrow_mut(|active| {
        let active = active.as_mut()?;
        if let Some(found) = active.resolved.get(request) {
            return found.clone();
        }
        let found = resolve(&active.theme, request);
        active.resolved.insert(request.to_owned(), found.clone());
        found
    })
}

fn resolve(theme: &gtk::IconTheme, request: &str) -> Option<PathBuf> {
    request
        .split(';')
        .filter(|name| !name.is_empty() && theme.has_icon(name))
        .find_map(|name| {
            let icon = theme.lookup_icon(
                name,
                &[],
                128,
                1,
                gtk::TextDirection::None,
                gtk::IconLookupFlags::FORCE_REGULAR,
            );
            icon.file()?.path().filter(|path| usable_theme_file(path))
        })
}

fn usable_theme_file(path: &Path) -> bool {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("");
    !stem.ends_with("-symbolic") && image_format(path).is_some()
}

pub fn installed_themes() -> Vec<InstalledTheme> {
    let mut roots = vec![
        glib::user_data_dir().join("icons"),
        glib::home_dir().join(".icons"),
    ];
    roots.extend(glib::system_data_dirs().iter().map(|dir| dir.join("icons")));
    let mut seen = HashSet::new();
    let mut themes = roots
        .iter()
        .filter_map(|root| std::fs::read_dir(root).ok())
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let id = entry.file_name().into_string().ok()?;
            let name = icon_theme_display_name(&entry.path().join("index.theme"))?;
            seen.insert(id.clone())
                .then_some(InstalledTheme { id, name })
        })
        .filter(|theme| !matches!(theme.id.as_str(), "hicolor" | "default" | "locolor"))
        .collect::<Vec<_>>();
    themes.sort_by_key(|theme| theme.name.to_lowercase());
    themes
}

/// `None` for cursor-only, hidden, or unreadable themes.
fn icon_theme_display_name(index: &Path) -> Option<String> {
    let file = glib::KeyFile::new();
    file.load_from_file(index, glib::KeyFileFlags::NONE).ok()?;
    let group = "Icon Theme";
    let hidden = file.boolean(group, "Hidden").unwrap_or(false);
    let has_directories = file
        .string(group, "Directories")
        .is_ok_and(|directories| !directories.trim().is_empty());
    if hidden || !has_directories {
        return None;
    }
    file.string(group, "Name").ok().map(|name| name.to_string())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ImageFormat {
    Svg,
    Png,
}

fn image_format(path: &Path) -> Option<ImageFormat> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    match extension.as_str() {
        "svg" => Some(ImageFormat::Svg),
        "png" => Some(ImageFormat::Png),
        _ => None,
    }
}

pub fn is_supported_image(path: &Path) -> bool {
    path.is_absolute() && image_format(path).is_some()
}

/// Renders a theme icon or a user-chosen image, fitted into a square.
pub(super) fn file_texture(path: &Path, texture_px: i32) -> Option<gdk::Texture> {
    let texture_px = texture_px.clamp(16, 768);
    let key = (
        path.to_string_lossy().into_owned(),
        String::new(),
        texture_px,
    );
    if let Some(texture) = super::cached_icon_texture(&key) {
        return Some(texture);
    }
    if std::fs::metadata(path).ok()?.len() > MAX_IMAGE_BYTES {
        return None;
    }
    let texture = match image_format(path)? {
        ImageFormat::Svg => svg_texture(path, texture_px),
        ImageFormat::Png => png_texture(path, texture_px),
    };
    match texture {
        Some(texture) => Some(super::cache_icon_texture(key, texture)),
        None => {
            tracing::debug!(path = %path.display(), "could not render icon");
            None
        }
    }
}

fn svg_texture(path: &Path, texture_px: i32) -> Option<gdk::Texture> {
    let source = std::fs::read(path).ok()?;
    let options = usvg::Options {
        resources_dir: None,
        image_href_resolver: usvg::ImageHrefResolver {
            resolve_data: Box::new(|_, _, _| None),
            resolve_string: Box::new(|_, _| None),
        },
        ..Default::default()
    };
    let tree = usvg::Tree::from_data(&source, &options).ok()?;
    let side = texture_px as u32;
    let mut pixmap = tiny_skia::Pixmap::new(side, side)?;
    let (width, height) = (tree.size().width(), tree.size().height());
    let scale = texture_px as f32 / width.max(height);
    let transform = tiny_skia::Transform::from_row(
        scale,
        0.0,
        0.0,
        scale,
        (texture_px as f32 - width * scale) / 2.0,
        (texture_px as f32 - height * scale) / 2.0,
    );
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Some(memory_texture(
        pixmap.take(),
        side,
        gdk::MemoryFormat::R8g8b8a8Premultiplied,
    ))
}

fn png_texture(path: &Path, texture_px: i32) -> Option<gdk::Texture> {
    let reader = image::ImageReader::open(path)
        .ok()?
        .with_guessed_format()
        .ok()?;
    let (width, height) = reader.into_dimensions().ok()?;
    if width.max(height) > MAX_IMAGE_DIMENSION || width == 0 || height == 0 {
        return None;
    }
    let decoded = image::ImageReader::open(path)
        .ok()?
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?
        .into_rgba8();
    let side = texture_px as u32;
    let scale = side as f32 / width.max(height) as f32;
    let fitted_width = ((width as f32 * scale).round() as u32).clamp(1, side);
    let fitted_height = ((height as f32 * scale).round() as u32).clamp(1, side);
    let fitted = image::imageops::resize(
        &decoded,
        fitted_width,
        fitted_height,
        image::imageops::FilterType::Triangle,
    );
    let mut canvas = image::RgbaImage::new(side, side);
    image::imageops::overlay(
        &mut canvas,
        &fitted,
        i64::from((side - fitted_width) / 2),
        i64::from((side - fitted_height) / 2),
    );
    Some(memory_texture(
        canvas.into_raw(),
        side,
        gdk::MemoryFormat::R8g8b8a8,
    ))
}

fn memory_texture(data: Vec<u8>, side: u32, format: gdk::MemoryFormat) -> gdk::Texture {
    gdk::MemoryTexture::new(
        side as i32,
        side as i32,
        format,
        &glib::Bytes::from_owned(data),
        side as usize * 4,
    )
    .upcast()
}

#[cfg(test)]
mod tests;

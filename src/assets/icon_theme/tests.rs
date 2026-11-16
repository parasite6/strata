// SPDX-License-Identifier: MIT

use std::path::Path;

use gtk::prelude::*;

use super::{
    BUILT_IN, builtin, file_icon, file_texture, folder_icon, icon_theme_display_name,
    installed_themes, select, themed_file,
};
use crate::assets::icons;

const RED_SQUARE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16"><rect width="16" height="16" fill="#ff0000"/></svg>"##;

fn write(path: &Path, contents: &[u8]) {
    std::fs::create_dir_all(path.parent().expect("parent")).expect("theme directory");
    std::fs::write(path, contents).expect("theme file");
}

fn install_test_theme() {
    let root = gtk::glib::user_data_dir().join("icons");
    write(
        &root.join("testtheme/index.theme"),
        b"[Icon Theme]\nName=Test Theme\nDirectories=scalable/mimetypes,scalable/places\n\n\
          [scalable/mimetypes]\nSize=64\nType=Scalable\nMinSize=8\nMaxSize=512\n\n\
          [scalable/places]\nSize=64\nType=Scalable\nMinSize=8\nMaxSize=512\n",
    );
    write(
        &root.join("testtheme/scalable/mimetypes/audio-x-generic.svg"),
        RED_SQUARE.as_bytes(),
    );
    write(
        &root.join("testtheme/scalable/mimetypes/text-x-generic-symbolic.svg"),
        RED_SQUARE.as_bytes(),
    );
    write(
        &root.join("testtheme/scalable/places/folder.svg"),
        RED_SQUARE.as_bytes(),
    );
    write(
        &root.join("pointers/index.theme"),
        b"[Icon Theme]\nName=Pointers\nInherits=Adwaita\n",
    );
}

fn center_pixel(texture: &gtk::gdk::Texture) -> [u8; 4] {
    let (width, height) = (texture.width() as usize, texture.height() as usize);
    let mut bytes = vec![0; width * height * 4];
    texture.download(&mut bytes, width * 4);
    let offset = (height / 2 * width + width / 2) * 4;
    [
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]
}

#[test]
fn builtin_recovers_the_bundled_icon_from_any_name() {
    assert_eq!(builtin(icons::FOLDER), icons::FOLDER);
    let token = format!("{}\u{1f}folder", icons::FOLDER);
    assert_eq!(builtin(&token), icons::FOLDER);
}

#[test]
fn custom_images_must_be_absolute_png_or_svg_paths() {
    assert!(icons::custom_image("image:/home/user/logo.svg").is_some());
    assert!(icons::custom_image("image:/home/user/logo.PNG").is_some());
    assert!(icons::custom_image("image:logo.svg").is_none());
    assert!(icons::custom_image("image:/home/user/photo.jpg").is_none());
    assert!(icons::custom_image("/home/user/logo.svg").is_none());
    assert!(icons::is_customization_choice("image:/home/user/logo.svg"));
}

#[test]
fn only_icon_themes_with_directories_are_listed() {
    let dir = tempfile::tempdir().expect("temp dir");
    let cases = [
        (
            "[Icon Theme]\nName=Kora\nDirectories=scalable/places\n",
            Some("Kora"),
        ),
        ("[Icon Theme]\nName=Cursors\nInherits=Adwaita\n", None),
        (
            "[Icon Theme]\nName=Hidden\nHidden=true\nDirectories=a\n",
            None,
        ),
        ("not a key file", None),
    ];
    for (contents, expected) in cases {
        let index = dir.path().join("index.theme");
        std::fs::write(&index, contents).expect("index");
        assert_eq!(
            icon_theme_display_name(&index).as_deref(),
            expected,
            "{contents}"
        );
    }
}

#[test]
fn selected_theme_draws_file_and_folder_icons_with_bundled_fallback() {
    crate::test_support::gtk_test(
        "assets::icon_theme::tests::selected_theme_draws_file_and_folder_icons_with_bundled_fallback",
        || {
            install_test_theme();
            let themes = installed_themes();
            assert!(
                themes
                    .iter()
                    .any(|theme| theme.id == "testtheme" && theme.name == "Test Theme")
            );
            assert!(themes.iter().all(|theme| theme.id != "pointers"));

            let song = file_icon(icons::FILE_AUDIO, "song.mp3");
            let notes = file_icon(icons::DOCUMENTS, "notes.txt");
            let folder = folder_icon(icons::FOLDER, None);
            assert_eq!(builtin(song), icons::FILE_AUDIO);
            assert!(
                themed_file(song).is_none(),
                "built-in theme draws bundled icons"
            );

            assert!(select("testtheme"));
            assert!(
                themed_file(song).is_some_and(|path| path.ends_with("audio-x-generic.svg")),
                "a generic theme icon wins over the bundled one"
            );
            assert!(themed_file(folder).is_some_and(|path| path.ends_with("folder.svg")));
            assert!(
                themed_file(notes).is_none(),
                "symbolic-only icons fall back"
            );

            let painted = crate::assets::tests::primary_icon_paintable(song).expect("theme icon");
            let [blue, green, red, alpha] = center_pixel(&painted);
            assert_eq!((red, green, blue, alpha), (255, 0, 0, 255));
            assert!(crate::assets::tests::primary_icon_paintable(notes).is_some());

            assert!(select(BUILT_IN));
            assert!(themed_file(song).is_none());
        },
    );
}

#[test]
fn custom_png_is_fitted_without_stretching() {
    crate::test_support::gtk_test(
        "assets::icon_theme::tests::custom_png_is_fitted_without_stretching",
        || {
            let dir = tempfile::tempdir().expect("temp dir");
            let path = dir.path().join("wide.png");
            image::RgbaImage::from_pixel(40, 20, image::Rgba([0, 0, 255, 255]))
                .save(&path)
                .expect("png");
            let texture = file_texture(&path, 64).expect("png texture");
            assert_eq!((texture.width(), texture.height()), (64, 64));
            let [blue, _, _, alpha] = center_pixel(&texture);
            assert_eq!((blue, alpha), (255, 255));
            let mut bytes = vec![0; 64 * 64 * 4];
            texture.download(&mut bytes, 64 * 4);
            assert_eq!(bytes[3], 0, "letterboxed corner stays transparent");

            let broken = dir.path().join("broken.svg");
            std::fs::write(&broken, "<svg").expect("svg");
            assert!(file_texture(&broken, 64).is_none());
        },
    );
}

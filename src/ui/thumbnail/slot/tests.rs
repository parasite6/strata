// SPDX-License-Identifier: MIT

use super::*;

#[test]
fn resizing_refreshes_fallbacks_without_discarding_decoded_thumbnails() {
    crate::test_support::gtk_test(
        "ui::thumbnail::slot::tests::resizing_refreshes_fallbacks_without_discarding_decoded_thumbnails",
        || {
            let slot = ThumbnailSlot::new(64);
            slot.set_icon_context(crate::assets::IconContext::Grid);
            crate::ui::thumbnail::show_fallback_icon(&slot, crate::assets::icons::FOLDER, 64);
            let small = slot
                .imp()
                .fallback
                .borrow()
                .clone()
                .expect("small fallback");
            slot.set_slot(128);
            let large = slot
                .imp()
                .fallback
                .borrow()
                .clone()
                .expect("large fallback");
            assert_ne!(small, large, "resizing must rerender fallback artwork");
            slot.set_slot(64);
            assert_eq!(slot.imp().fallback.borrow().as_ref(), Some(&small));
            slot.set_texture(&large);
            slot.set_slot(128);
            assert_eq!(slot.texture(), Some(large));
        },
    );
}

#[test]
fn folder_previews_replace_and_clear_children() {
    crate::test_support::gtk_test(
        "ui::thumbnail::slot::tests::folder_previews_replace_and_clear_children",
        || {
            let folder = ThumbnailSlot::new(64);
            let (a, b, c) = (
                ThumbnailSlot::new(32),
                ThumbnailSlot::new(32),
                ThumbnailSlot::new(32),
            );
            folder.set_folder_previews(&[a.clone(), b.clone()]);
            assert_eq!(folder.folder_preview_children(), [a.clone(), b.clone()]);
            assert!(
                [&a, &b]
                    .iter()
                    .all(|child| child.parent().as_ref() == Some(folder.upcast_ref()))
            );
            folder.set_folder_previews(std::slice::from_ref(&c));
            assert!(a.parent().is_none() && b.parent().is_none());
            assert_eq!(folder.folder_preview_children(), std::slice::from_ref(&c));
            folder.clear_folder_previews();
            assert!(folder.folder_preview_children().is_empty());
            assert!(c.parent().is_none());
        },
    );
}

// SPDX-License-Identifier: MIT

use gtk::prelude::*;
use gtk::{gio, glib};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::properties_row_with_suffix;

/// Adds a SHA-256 row for the regular file at `path`, hashed off the interface
/// thread. Hashing stops when `layer` closes.
pub(super) fn add_checksum_row(details: &gtk::Box, layer: &gtk::Widget, path: PathBuf) {
    let spinner = gtk::Spinner::new();
    spinner.add_css_class("properties-size-spinner");
    spinner.set_valign(gtk::Align::Center);
    spinner.set_halign(gtk::Align::End);
    crate::ui::accessibility::set_label(&spinner, &crate::i18n::tr("Calculating checksum"));
    spinner.set_spinning(true);
    let value = properties_row_with_suffix(
        details,
        "SHA-256",
        &crate::i18n::tr("Calculating…"),
        Some(spinner.upcast_ref()),
    );
    value.add_css_class("properties-checksum-value");
    value.set_ellipsize(gtk::pango::EllipsizeMode::None);
    value.set_wrap(true);
    value.set_wrap_mode(gtk::pango::WrapMode::Char);
    value.set_selectable(true);

    let cancelled = Arc::new(AtomicBool::new(false));
    let on_close = cancelled.clone();
    layer.connect_unrealize(move |_| on_close.store(true, Ordering::Relaxed));
    let on_dismiss = cancelled.clone();
    layer.connect_sensitive_notify(move |layer| {
        if !layer.is_sensitive() {
            on_dismiss.store(true, Ordering::Relaxed);
        }
    });

    glib::MainContext::default().spawn_local(async move {
        let worker_cancelled = cancelled.clone();
        let result =
            gio::spawn_blocking(move || crate::services::sha256_file(&path, &worker_cancelled))
                .await;
        if cancelled.load(Ordering::Relaxed) {
            return;
        }
        spinner.set_spinning(false);
        spinner.set_visible(false);
        match result {
            Ok(Ok(Some(hash))) => value.set_text(&hash),
            _ => value.set_text(&crate::i18n::tr("Unavailable")),
        }
    });
}

// SPDX-License-Identifier: MIT

//! The icon theme choice: bundled icons, the system theme, or an installed theme.

use std::rc::Rc;

use gtk::prelude::*;

use crate::{
    assets::icon_theme::{self, BUILT_IN, FOLLOW_SYSTEM},
    ui::preferences::PreferenceManager,
};

pub(super) fn append_icon_theme_option(group: &gtk::Box, preferences: &Rc<PreferenceManager>) {
    let choices = icon_theme_choices();
    let choice = icon_theme_menu(preferences, &choices);
    group.append(&super::control_row(
        "Icon theme",
        "File and folder icons from an installed icon theme. File icon colors only apply to Strata's built-in icons.",
        &choice,
    ));
}

/// Labels of the first two entries are translated; theme names come from the theme itself.
fn icon_theme_choices() -> Vec<(String, String)> {
    let mut choices = vec![
        (BUILT_IN.to_owned(), crate::i18n::tr("Built-in")),
        (FOLLOW_SYSTEM.to_owned(), crate::i18n::tr("Follow system")),
    ];
    choices.extend(
        icon_theme::installed_themes()
            .into_iter()
            .map(|theme| (theme.id, theme.name)),
    );
    choices
}

fn choice_label(choices: &[(String, String)], value: &str) -> String {
    choices
        .iter()
        .find(|(id, _)| id == value)
        .map_or_else(|| value.to_owned(), |(_, label)| label.clone())
}

fn icon_theme_menu(
    preferences: &Rc<PreferenceManager>,
    choices: &[(String, String)],
) -> gtk::MenuButton {
    let menu = gtk::Box::new(gtk::Orientation::Vertical, 2);
    menu.add_css_class("column-menu");
    let scroller = gtk::ScrolledWindow::builder()
        .child(&menu)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .max_content_height(360)
        .build();
    let popover = gtk::Popover::builder()
        .child(&scroller)
        .has_arrow(false)
        .build();
    popover.add_css_class("column-popover");
    let button = gtk::MenuButton::builder()
        .popover(&popover)
        .always_show_arrow(true)
        .valign(gtk::Align::Center)
        .build();
    button.add_css_class("form-control");
    button.add_css_class("settings-choice");
    crate::ui::accessibility::set_label(&button, &crate::i18n::tr("Icon theme"));
    let labels = choices.to_vec();
    preferences.bind_preference(
        &button,
        PreferenceManager::icon_theme,
        move |widget, value| {
            if let Some(button) = widget.downcast_ref::<gtk::MenuButton>() {
                super::bindings::show_choice_value(button, &choice_label(&labels, &value));
            }
        },
    );
    for (id, label) in choices {
        menu.append(&icon_theme_option(preferences, &button, id, label));
    }
    button
}

fn icon_theme_option(
    preferences: &Rc<PreferenceManager>,
    button: &gtk::MenuButton,
    id: &str,
    label: &str,
) -> gtk::Button {
    let selected = preferences.icon_theme() == id;
    let (option, check) = crate::ui::controls::menu_option(label, selected);
    let value = id.to_owned();
    preferences.bind_preference(
        &check,
        PreferenceManager::icon_theme,
        move |widget, current| {
            widget.set_visible(current == value);
        },
    );
    let weak_button = button.downgrade();
    let manager = preferences.clone();
    let value = id.to_owned();
    option.connect_clicked(move |_| {
        manager.set_icon_theme(&value);
        if let Some(button) = weak_button.upgrade() {
            button.popdown();
        }
    });
    option
}

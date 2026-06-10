// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/get_software/mod.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gio::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

pub use package::PackageObject;

mod imp;
mod package;

glib::wrapper! {
    pub struct StackPageGetSoftware(ObjectSubclass<imp::StackPageGetSoftware>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}

impl StackPageGetSoftware {
    pub fn connect_navigate<F: Fn(&Self, &str) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "navigate",
            true,
            glib::closure_local!(move |obj: Self, stackpage: &str| {
                f(&obj, stackpage);
            }),
        )
    }
    pub fn toggle_column_view(&self) {
        let column_view_packages = &self.imp().column_view_packages;
        let scrolled_window_packages = column_view_packages
            .parent()
            .and_downcast::<gtk::ScrolledWindow>()
            .unwrap();

        let text_view_log = &self.imp().text_view_log;
        let scrolled_window_log = text_view_log
            .parent()
            .and_downcast::<gtk::ScrolledWindow>()
            .unwrap();

        if scrolled_window_packages.is_visible() {
            scrolled_window_packages.set_visible(false);
            scrolled_window_log.set_visible(true);
        } else {
            scrolled_window_packages.set_visible(true);
            scrolled_window_log.set_visible(false);
        }
    }
}

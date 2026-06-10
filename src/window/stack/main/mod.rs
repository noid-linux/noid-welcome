// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/main/mod.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gio::prelude::*;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct StackPageMain(ObjectSubclass<imp::StackPageMain>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}

impl StackPageMain {
    pub fn connect_run_tweak<F: Fn(&Self, &str, &str) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_closure(
            "run-tweak",
            true,
            glib::closure_local!(move |obj: Self, title: &str, summary: &str| {
                f(&obj, title, summary);
            }),
        )
    }

    pub fn connect_navigate<F: Fn(&Self, &str) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "navigate",
            true,
            glib::closure_local!(move |obj: Self, stackpage: &str| {
                f(&obj, stackpage);
            }),
        )
    }
}
